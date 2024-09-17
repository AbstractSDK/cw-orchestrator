use http_body_util::BodyExt;

use crate::CwEnvError;

#[derive(Debug, Clone)]
pub struct GithubWasmPath {
    pub owner: String,
    pub repo_name: String,
    pub location: GithubWasmPathLocation,
}

#[derive(Debug, Clone)]
pub enum GithubWasmPathLocation {
    /// Corresponds to a wasm file attached to a release
    Release { tag: String, file_name: String },
    /// Corresponds to a file within a repo. Reference can be :
    /// - A tag
    /// - A commit number
    /// - A branch
    File {
        reference: String,
        file_path: String,
    },
}

impl GithubWasmPath {
    pub async fn wasm(&self) -> Result<Vec<u8>, CwEnvError> {
        match &self.location {
            super::github::GithubWasmPathLocation::Release { tag, file_name } => {
                let release = octocrab::instance()
                    .repos(&self.owner, &self.repo_name)
                    .releases()
                    .get_by_tag(tag)
                    .await
                    .map_err(|e| CwEnvError::Octocrab(e.to_string()))?;
                let wasm_asset = release
                    .assets
                    .iter()
                    .find(|asset| asset.name.eq(file_name))
                    .ok_or(CwEnvError::ReleaseArtifactNotFound(file_name.to_string()))?;
                let response = reqwest::get(wasm_asset.browser_download_url.clone()).await?;
                let content = response.bytes().await?;
                Ok(content.to_vec())
            }
            super::github::GithubWasmPathLocation::File {
                reference,
                file_path,
            } => {
                let mut response = octocrab::instance()
                    .repos(&self.owner, &self.repo_name)
                    .raw_file(reference.clone(), file_path)
                    .await
                    .map_err(|e| CwEnvError::Octocrab(e.to_string()))?;

                let body = response.body_mut();
                let mut bytes = Vec::new();

                while let Some(chunk) = body.frame().await {
                    let chunk = chunk.map_err(|e| CwEnvError::Octocrab(e.to_string()))?;
                    bytes.extend_from_slice(&chunk.into_data().unwrap());
                }
                Ok(bytes.to_vec())
            }
        }
    }
}
