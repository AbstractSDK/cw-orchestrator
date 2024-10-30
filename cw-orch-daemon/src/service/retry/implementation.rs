use super::Policy;
use futures::TryStreamExt;
use futures_util::future;
use http::{request::Parts, Request};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use tonic::body::BoxBody;

type Req = http::Request<BoxBody>;
type Res = http::Response<BoxBody>;

#[derive(Clone)]
pub struct Attempts(pub usize);

impl<E> Policy<Req, Res, E> for Attempts {
    type Future = future::Ready<()>;

    fn retry(&mut self, _req: &mut Req, result: &mut Result<Res, E>) -> Option<Self::Future> {
        match result {
            Ok(_) => {
                log::trace!("Entering the middleware ok");
                // Treat all `Response`s as success,
                // so don't retry...
                None
            }
            Err(_) => {
                log::trace!("Entering the middleware error");
                // Treat all errors as failures...
                // But we limit the number of attempts...
                if self.0 > 0 {
                    log::trace!("Try this again, there was a failure");
                    // Try again!
                    self.0 -= 1;
                    Some(future::ready(()))
                } else {
                    // Used all our attempts, no retry...
                    None
                }
            }
        }
    }

    fn clone_request(&mut self, req: Req) -> (Req, Option<Req>) {
        // Convert body to Bytes so it can be cloned
        let (parts, original_body) = req.into_parts();

        // Try to capture the Bytes from the original body
        // This is circumvoluted, I'm not sure how to call an async function within a sync function that is used inside a future later
        let bytes = futures::executor::block_on(async move {
            tokio::runtime::Handle::current()
                .spawn(async move { consume_unsync_body(original_body).await })
                .await
                .unwrap()
        });

        // Re-create the request with the captured bytes in a new BoxBody
        let req = create_request(parts.clone(), bytes.clone());
        let cloned_req = create_request(parts, bytes);

        (req, Some(cloned_req))
        // Some(req.clone())
    }
}

async fn consume_unsync_body(body: BoxBody) -> Vec<u8> {
    // Accumulate bytes asynchronously

    body.into_data_stream()
        .try_fold(Vec::new(), |mut acc, chunk| async move {
            acc.extend_from_slice(&chunk);
            Ok(acc)
        })
        .await
        .unwrap()
}

fn create_request(parts: Parts, body: Vec<u8>) -> http::Request<BoxBody> {
    let bytes = Bytes::from(body);
    let full_body = Full::new(bytes);
    let mut request = Request::builder()
        .method(parts.method)
        .uri(parts.uri)
        .version(parts.version)
        .body(
            full_body
                .map_err(|_err| tonic::Status::internal("Body error"))
                .boxed_unsync(),
        )
        .unwrap();

    *request.headers_mut() = parts.headers;

    request
}
