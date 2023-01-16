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
///     fn deploy_on(chain: Chain, version: impl Into<String>) -> Result<Self, BootError> {
///         let token = Cw20::new("my-token", chain.clone());
///         self.token.upload()?;
///         self.token.instantiate(...)?;
///         Ok(Self { token })
///     }
/// }
/// ```
/// 
/// This allows other developers to re-use the application's deployment logic in their own tests. 
/// Allowing them to build on the application's functionality without having to re-implement its deployment.
pub trait Deploy<Chain: BootEnvironment>: Sized {
    type Error: From<BootError>;
    /// Deploy the application to the chain.
    fn deploy_on(chain: Chain, version: impl Into<String>) -> Result<Self, Self::Error>;
}