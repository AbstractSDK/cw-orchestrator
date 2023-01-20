use crate::{BootError, BootEnvironment};


/// Indicates the ability to deploy an application to a mock chain.
/// 
/// ## Example:
/// ```rust
/// use boot_core::mock::deploy::Deploy;
/// 
/// pub struct MyApplication<Chain: BootEnvironment> {
///   pub token: Cw20<Chain>
/// }
/// 
/// impl Deploy for MyApplication<Mock> {
///     type Error = BootError;
///     type DeployData = Empty;
///     // deploys the token to the chain
///     fn deploy_on(chain: Chain, data: Empty) -> Result<R, BootError> {
///         let token = Cw20::new("my-token", chain.clone());
///         self.token.upload()?;
///         self.token.instantiate(...)?;
///         Ok(Self { token })
///     }
///    // loads the token from the chain
///    fn load_from(chain: Chain) -> Result<R, BootError> {
///         let token = Cw20::new("my-token", chain.clone());
///         Ok(Self { token })
///    }
/// }
/// ```
/// 
/// This allows other developers to re-use the application's deployment logic in their own tests. 
/// Allowing them to build on the application's functionality without having to re-implement its deployment.
pub trait Deploy<Chain: BootEnvironment>: Sized {
    type Error: From<BootError>;
    /// Data required to deploy the application.
    type DeployData;
    /// Deploy the application to the chain.
    fn deploy_on(chain: Chain, data: Self::DeployData) -> Result<Self, Self::Error>;
    /// Load the application from the chain, assuming it has already been deployed.
    /// This either loads contract addresses from the chain state manually or constructs the
    /// boot contract wrappers that were used to deploy the application with the same name.
    fn load_from(chain: Chain) -> Result<Self, Self::Error>;
}