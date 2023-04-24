use crate::{BootError, CwEnv};

/// Indicates the ability to deploy an application to a mock chain.
///
/// ## Example:
/// ```ignore
/// use boot_core::{Deploy, BootError, Empty, CwEnv, BootUpload};
/// use boot_cw_plus::Cw20Base;
/// use cw20::Cw20Coin;
///
/// pub struct MyApplication<Chain: CwEnv> {
///   pub token: Cw20Base<Chain>
/// }
///
/// impl<Chain: CwEnv> Deploy<Chain> for MyApplication<Chain> {
///     type Error = BootError;
///     type DeployData = Empty;
///     fn store_on(chain: Chain) -> Result<Self, BootError> {
///         let mut token = Cw20Base::new("my-token", chain.clone());
///         token.upload()?;
///         Ok(Self { token })
///     }
///
///     // deploys the token to the chain
///     fn deploy_on(chain: Chain, data: Empty) -> Result<Self, BootError> {
///         let my_app: MyApplication<Chain> = Self::store_on(chain)?;
///         let cw20_init_msg = cw20_base::msg::InstantiateMsg {
///             decimals: 6,
///             name: "Test Token".to_string(),
///             initial_balances: vec![],
///             marketing: None,
///             mint: None,
///             symbol: "TEST".to_string(),
///         };
///         // instantiates the token and stores its address to the "my-token" key
///         my_app.token.instantiate(&cw20_init_msg, None, None)?;
///         Ok(my_app)
///    }
///
///    // loads the token from the chain
///    fn load_from(chain: Chain) -> Result<Self, BootError> {
///        // loads the token and uses the "my-token" key to get its information
///         let token = Cw20Base::new("my-token", chain.clone());
///         Ok(Self { token })
///    }
/// }
/// ```
///
/// This allows other developers to re-use the application's deployment logic in their own tests.
/// Allowing them to build on the application's functionality without having to re-implement its deployment.
pub trait Deploy<Chain: CwEnv>: Sized {
    type Error: From<BootError>;

    /// Data required to deploy the application.
    type DeployData;

    /// Stores/uploads the application to the chain.
    fn store_on(chain: Chain) -> Result<Self, Self::Error>;

    /// Deploy the application to the chain. This could include instantiating contracts.
    #[allow(unused_variables)]
    fn deploy_on(chain: Chain, data: Self::DeployData) -> Result<Self, Self::Error> {
        // if not implemented, just store the application on the chain
        Self::store_on(chain)
    }

    /// Load the application from the chain, assuming it has already been deployed.
    fn load_from(chain: Chain) -> Result<Self, Self::Error>;
}
