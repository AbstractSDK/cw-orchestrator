use crate::{BootError, BootEnvironment};


/// Indicates the ability to deploy an application to a mock chain.
/// 
/// ## Example:
/// ```rust
/// use boot_core::mock::deploy::Deploy;
/// 
/// pub struct MyApplication<Chain: BootEnvironment> {
///   pub chain: Chain,
///   pub token: Cw20<Chain>
/// }
/// 
/// impl Deploy for MyApplication<Mock> {
///     fn deploy(&mut self) -> Result<(), Error> {
///        self.token.upload()?;
///        self.token.instantiate(...)?;
///     }
/// }
/// ```
/// 
/// This allows other developers to re-use the application's deployment logic in their own tests. 
/// Allowing them to build on the application's functionality without having to re-implement its deployment.
pub trait Deploy<Chain: BootEnvironment>: Sized {
    type Error: From<BootError>;
    /// Deploy the application to the mockchain.
    fn deploy_on(chain: Chain) -> Result<Self, Self::Error>;
}