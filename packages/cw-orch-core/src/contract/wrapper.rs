//! # Implementation of the contract trait and contract wrapper

use anyhow::{anyhow, bail, Error as AnyError, Result as AnyResult};
use cosmwasm_std::{
    from_json, Binary, Checksum, CosmosMsg, CustomMsg, CustomQuery, Deps, DepsMut, Empty, Env,
    MessageInfo, QuerierWrapper, Reply, Response, SubMsg,
};
use cosmwasm_std::{
    IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg,
    IbcChannelOpenResponse, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse,
};
use serde::de::DeserializeOwned;
use std::fmt::{Debug, Display};
use std::ops::Deref;

/// This trait serves as a primary interface for interacting with contracts.
#[rustfmt::skip]
pub trait Contract<C, Q = Empty>
where
    C: CustomMsg,
    Q: CustomQuery,
{
    /// Evaluates contract's `execute` entry-point.
    fn execute(&self, deps: DepsMut<Q>, env: Env, info: MessageInfo, msg: Vec<u8>) -> AnyResult<Response<C>>;

    /// Evaluates contract's `instantiate` entry-point.
    fn instantiate(&self, deps: DepsMut<Q>, env: Env, info: MessageInfo, msg: Vec<u8>) -> AnyResult<Response<C>>;

    /// Evaluates contract's `query` entry-point.
    fn query(&self, deps: Deps<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Binary>;

    /// Evaluates contract's `sudo` entry-point.
    fn sudo(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>>;

    /// Evaluates contract's `reply` entry-point.
    fn reply(&self, deps: DepsMut<Q>, env: Env, msg: Reply) -> AnyResult<Response<C>>;

    /// Evaluates contract's `migrate` entry-point.
    fn migrate(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>>;

    /// Returns the provided checksum of the contract's Wasm blob.
    fn checksum(&self) -> Option<Checksum> {
        None
    }

    /// Executes the contract ibc_channel_open endpoint
    #[allow(unused)]
    fn ibc_channel_open(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcChannelOpenMsg,
    ) -> AnyResult<IbcChannelOpenResponse> {
        bail!("No Ibc capabilities on this contract")
    }

    /// Executes the contract ibc_channel_connect endpoint
    #[allow(unused)]
    fn ibc_channel_connect(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcChannelConnectMsg,
    ) -> AnyResult<IbcBasicResponse<C>> {
        bail!("No Ibc capabilities on this contract")
    }

    /// Executes the contract ibc_channel_close endpoint
    #[allow(unused)]
    fn ibc_channel_close(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcChannelCloseMsg,
    ) -> AnyResult<IbcBasicResponse<C>> {
        bail!("No Ibc capabilities on this contract")
    }

    /// Executes the contract ibc_packet_receive endpoint
    #[allow(unused)]
    fn ibc_packet_receive(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcPacketReceiveMsg,
    ) -> AnyResult<IbcReceiveResponse<C>> {
        bail!("No Ibc capabilities on this contract")
    }

    /// Executes the contract ibc_packet_acknowledge endpoint
    #[allow(unused)]
    fn ibc_packet_acknowledge(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcPacketAckMsg,
    ) -> AnyResult<IbcBasicResponse<C>> {
        bail!("No Ibc capabilities on this contract")
    }

    /// Executes the contract ibc_packet_timeout endpoint
    #[allow(unused)]
    fn ibc_packet_timeout(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcPacketTimeoutMsg,
    ) -> AnyResult<IbcBasicResponse<C>> {
        bail!("No Ibc capabilities on this contract")
    }
}

#[rustfmt::skip]
mod closures {
    use super::*;

    // function types
    pub type IbcFn<T, R, E, Q> = fn(deps: DepsMut<Q>, env: Env, msg: T) -> Result<R, E>;

    pub type ContractFn<T, C, E, Q> = fn(deps: DepsMut<Q>, env: Env, info: MessageInfo, msg: T) -> Result<Response<C>, E>;
    pub type PermissionedFn<T, C, E, Q> = fn(deps: DepsMut<Q>, env: Env, msg: T) -> Result<Response<C>, E>;
    pub type ReplyFn<C, E, Q> = fn(deps: DepsMut<Q>, env: Env, msg: Reply) -> Result<Response<C>, E>;
    pub type QueryFn<T, E, Q> = fn(deps: Deps<Q>, env: Env, msg: T) -> Result<Binary, E>;

    // closure types
    pub type IbcClosure<T, R, E, Q> = Box<dyn Fn(DepsMut<Q>,Env, T) -> Result<R, E>>;

    pub type ContractClosure<T, C, E, Q> = Box<dyn Fn(DepsMut<Q>, Env, MessageInfo, T) -> Result<Response<C>, E>>;
    pub type PermissionedClosure<T, C, E, Q> = Box<dyn Fn(DepsMut<Q>, Env, T) -> Result<Response<C>, E>>;
    pub type ReplyClosure<C, E, Q> = Box<dyn Fn(DepsMut<Q>, Env, Reply) -> Result<Response<C>, E>>;
    pub type QueryClosure<T, E, Q> = Box<dyn Fn(Deps<Q>, Env, T) -> Result<Binary, E>>;
}

use closures::*;

/// This structure wraps the [Contract] trait implementor
/// and provides generic access to the contract's entry-points.
///
/// List of generic types used in [ContractWrapper]:
/// - **T1** type of message passed to [execute] entry-point.
/// - **T2** type of message passed to [instantiate] entry-point.
/// - **T3** type of message passed to [query] entry-point.
/// - **T4** type of message passed to [sudo] entry-point.
/// - instead of **~~T5~~**, always the `Reply` type is used in [reply] entry-point.
/// - **T6** type of message passed to [migrate] entry-point.
/// - **E1** type of error returned from [execute] entry-point.
/// - **E2** type of error returned from [instantiate] entry-point.
/// - **E3** type of error returned from [query] entry-point.
/// - **E4** type of error returned from [sudo] entry-point.
/// - **E5** type of error returned from [reply] entry-point.
/// - **E6** type of error returned from [migrate] entry-point.
/// - **C** type of custom message returned from all entry-points except [query].
/// - **Q** type of custom query in `Querier` passed as 'Deps' or 'DepsMut' to all entry-points.
///
/// The following table summarizes the purpose of all generic types used in [ContractWrapper].
/// ```text
/// ┌─────────────────────┬────────────────────────┬─────────────────────┬─────────┬─────────┬───────┬───────┐
/// │      Contract       │    Contract            │                     │         │         │       │       │
/// │    entry-point      │    wrapper             │    Closure type     │ Message │ Message │ Error │ Query │
/// │                     │    member              │                     │   IN    │   OUT   │  OUT  │       │
/// ╞═════════════════════╪════════════════════════╪═════════════════════╪═════════╪═════════╪═══════╪═══════╡
/// │          (1)        │                        │                     │         │         │       │       │
/// ╞═════════════════════╪════════════════════════╪═════════════════════╪═════════╪═════════╪═══════╪═══════╡
/// │ execute             │ execute_fn             │ ContractClosure     │   T1    │    C    │  E1   │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ instantiate         │ instantiate_fn         │ ContractClosure     │   T2    │    C    │  E2   │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ query               │ query_fn               │ QueryClosure        │   T3    │ Binary  │  E3   │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ sudo                │ sudo_fn                │ PermissionedClosure │   T4    │    C    │  E4   │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ reply               │ reply_fn               │ ReplyClosure        │  Reply  │    C    │  E5   │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ migrate             │ migrate_fn             │ PermissionedClosure │   T6    │    C    │  E6   │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ ibc_channel_open    │ ibc_channel_open_fn    │ IbcClosure          │   --    │    C    │  E7   │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ ibc_channel_connect │ ibc_channel_connect_fn │ IbcClosure          │   --    │    C    │  E8   │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ ibc_channel_close   │ ibc_channel_close_fn   │ IbcClosure          │   --    │    C    │  E9   │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ ibc_packet_receive  │ ibc_packet_receive_fn  │ IbcClosure          │   --    │    C    │  E10  │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ ibc_packet_ack      │ ibc_packet_ack_fn      │ IbcClosure          │   --    │    C    │  E11  │   Q   │
/// ├─────────────────────┼────────────────────────┼─────────────────────┼─────────┼─────────┼───────┼───────┤
/// │ ibc_packet_timeout  │ ibc_packet_timeout_fn  │ IbcClosure          │   --    │    C    │  E12  │   Q   │
/// └─────────────────────┴────────────────────────┴─────────────────────┴─────────┴─────────┴───────┴───────┘
/// ```
/// The general schema depicting which generic type is used in entry points is shown below.
/// Entry point, when called, is provided minimum two arguments: custom query of type **Q**
/// (inside `Deps` or `DepsMut`) and input message of type **T1**, **T2**, **T3**, **T4**,
/// **Reply** or **T6**. As a result, entry point returns custom output message of type
/// Response<**C**> or **Binary** and an error of type **E1**, **E2**, **E3**, **E4**, **E5**
/// or **E6**.
///
/// ```text
///    entry_point(query, .., message_in) -> Result<message_out, error>
///                  ┬           ┬                      ┬          ┬
///             Q >──┘           │                      │          └──> E1,E2,E3,E4,E5,E6
///    T1,T2,T3,T4,Reply,T6 >────┘                      └─────────────> C,Binary
/// ```
/// Generic type **C** defines a custom message that is specific for the **whole blockchain**.
/// Similarly, the generic type **Q** defines a custom query that is also specific
/// to the **whole blockchain**. Other generic types are specific to the implemented contract.
/// So all smart contracts used in the same blockchain will have the same types for **C** and **Q**,
/// but each contract may use different type for other generic types.
/// It means that e.g. **T1** in smart contract `A` may differ from **T1** in smart contract `B`.
///
/// [execute]: Contract::execute
/// [instantiate]: Contract::instantiate
/// [query]: Contract::query
/// [sudo]: Contract::sudo
/// [reply]: Contract::reply
/// [migrate]: Contract::migrate
pub struct ContractWrapper<
    T1,
    T2,
    T3,
    E1,
    E2,
    E3,
    C = Empty,
    Q = Empty,
    T4 = Empty,
    E4 = AnyError,
    E5 = AnyError,
    T6 = Empty,
    E6 = AnyError,
    E7 = AnyError,
    E8 = AnyError,
    E9 = AnyError,
    E10 = AnyError,
    E11 = AnyError,
    E12 = AnyError,
> where
    T1: DeserializeOwned, // Type of message passed to `execute` entry-point.
    T2: DeserializeOwned, // Type of message passed to `instantiate` entry-point.
    T3: DeserializeOwned, // Type of message passed to `query` entry-point.
    T4: DeserializeOwned, // Type of message passed to `sudo` entry-point.
    T6: DeserializeOwned, // Type of message passed to `migrate` entry-point.
    E1: Display + Debug + Send + Sync, // Type of error returned from `execute` entry-point.
    E2: Display + Debug + Send + Sync, // Type of error returned from `instantiate` entry-point.
    E3: Display + Debug + Send + Sync, // Type of error returned from `query` entry-point.
    E4: Display + Debug + Send + Sync, // Type of error returned from `sudo` entry-point.
    E5: Display + Debug + Send + Sync, // Type of error returned from `reply` entry-point.
    E6: Display + Debug + Send + Sync, // Type of error returned from `migrate` entry-point.
    E7: Display + Debug + Send + Sync, // Type of error returned from `channel_open` entry-point.
    E8: Display + Debug + Send + Sync, // Type of error returned from `channel_connect` entry-point.
    E9: Display + Debug + Send + Sync, // Type of error returned from `channel_close` entry-point.
    E10: Display + Debug + Send + Sync, // Type of error returned from `ibc_packet_receive` entry-point.
    E11: Display + Debug + Send + Sync, // Type of error returned from `ibc_packet_ack` entry-point.
    E12: Display + Debug + Send + Sync, // Type of error returned from `ibc_packet_timeout` entry-point.
    C: CustomMsg, // Type of custom message returned from all entry-points except `query`.
    Q: CustomQuery + DeserializeOwned, // Type of custom query in querier passed as deps/deps_mut to all entry-points.
{
    execute_fn: ContractClosure<T1, C, E1, Q>,
    instantiate_fn: ContractClosure<T2, C, E2, Q>,
    query_fn: QueryClosure<T3, E3, Q>,
    sudo_fn: Option<PermissionedClosure<T4, C, E4, Q>>,
    reply_fn: Option<ReplyClosure<C, E5, Q>>,
    migrate_fn: Option<PermissionedClosure<T6, C, E6, Q>>,
    checksum: Option<Checksum>,

    channel_open_fn: Option<IbcClosure<IbcChannelOpenMsg, IbcChannelOpenResponse, E7, Q>>,
    channel_connect_fn: Option<IbcClosure<IbcChannelConnectMsg, IbcBasicResponse<C>, E8, Q>>,
    channel_close_fn: Option<IbcClosure<IbcChannelCloseMsg, IbcBasicResponse<C>, E9, Q>>,

    ibc_packet_receive_fn: Option<IbcClosure<IbcPacketReceiveMsg, IbcReceiveResponse<C>, E10, Q>>,
    ibc_packet_ack_fn: Option<IbcClosure<IbcPacketAckMsg, IbcBasicResponse<C>, E11, Q>>,
    ibc_packet_timeout_fn: Option<IbcClosure<IbcPacketTimeoutMsg, IbcBasicResponse<C>, E12, Q>>,
}

impl<T1, T2, T3, E1, E2, E3, C, Q> ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q>
where
    T1: DeserializeOwned + 'static, // Type of message passed to `execute` entry-point.
    T2: DeserializeOwned + 'static, // Type of message passed to `instantiate` entry-point.
    T3: DeserializeOwned + 'static, // Type of message passed to `query` entry-point.
    E1: Display + Debug + Send + Sync + 'static, // Type of error returned from `execute` entry-point.
    E2: Display + Debug + Send + Sync + 'static, // Type of error returned from `instantiate` entry-point.
    E3: Display + Debug + Send + Sync + 'static, // Type of error returned from `query` entry-point.
    C: CustomMsg + 'static, // Type of custom message returned from all entry-points except `query`.
    Q: CustomQuery + DeserializeOwned + 'static, // Type of custom query in querier passed as deps/deps_mut to all entry-points.
{
    /// Creates a new contract wrapper with default settings.
    pub fn new(
        execute_fn: ContractFn<T1, C, E1, Q>,
        instantiate_fn: ContractFn<T2, C, E2, Q>,
        query_fn: QueryFn<T3, E3, Q>,
    ) -> Self {
        Self {
            execute_fn: Box::new(execute_fn),
            instantiate_fn: Box::new(instantiate_fn),
            query_fn: Box::new(query_fn),
            sudo_fn: None,
            reply_fn: None,
            migrate_fn: None,
            checksum: None,

            channel_open_fn: None,
            channel_connect_fn: None,
            channel_close_fn: None,

            ibc_packet_receive_fn: None,
            ibc_packet_ack_fn: None,
            ibc_packet_timeout_fn: None,
        }
    }

    /// This will take a contract that returns `Response<Empty>` and will _upgrade_ it
    /// to `Response<C>` if needed, to be compatible with a chain-specific extension.
    pub fn new_with_empty(
        execute_fn: ContractFn<T1, Empty, E1, Empty>,
        instantiate_fn: ContractFn<T2, Empty, E2, Empty>,
        query_fn: QueryFn<T3, E3, Empty>,
    ) -> Self {
        Self {
            execute_fn: customize_contract_fn(execute_fn),
            instantiate_fn: customize_contract_fn(instantiate_fn),
            query_fn: customize_query_fn(query_fn),
            sudo_fn: None,
            reply_fn: None,
            migrate_fn: None,
            checksum: None,

            channel_open_fn: None,
            channel_connect_fn: None,
            channel_close_fn: None,

            ibc_packet_receive_fn: None,
            ibc_packet_ack_fn: None,
            ibc_packet_timeout_fn: None,
        }
    }
}

#[allow(clippy::type_complexity)]
impl<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5, T6, E6>
    ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5, T6, E6>
where
    T1: DeserializeOwned, // Type of message passed to `execute` entry-point.
    T2: DeserializeOwned, // Type of message passed to `instantiate` entry-point.
    T3: DeserializeOwned, // Type of message passed to `query` entry-point.
    T4: DeserializeOwned, // Type of message passed to `sudo` entry-point.
    T6: DeserializeOwned, // Type of message passed to `migrate` entry-point.
    E1: Display + Debug + Send + Sync, // Type of error returned from `execute` entry-point.
    E2: Display + Debug + Send + Sync, // Type of error returned from `instantiate` entry-point.
    E3: Display + Debug + Send + Sync, // Type of error returned from `query` entry-point.
    E4: Display + Debug + Send + Sync, // Type of error returned from `sudo` entry-point.
    E5: Display + Debug + Send + Sync, // Type of error returned from `reply` entry-point.
    E6: Display + Debug + Send + Sync, // Type of error returned from `migrate` entry-point.
    C: CustomMsg + 'static, // Type of custom message returned from all entry-points except `query`.
    Q: CustomQuery + DeserializeOwned + 'static, // Type of custom query in querier passed as deps/deps_mut to all entry-points.
{
    /// Populates [ContractWrapper] with contract's `sudo` entry-point and custom message type.
    pub fn with_sudo<T4A, E4A>(
        self,
        sudo_fn: PermissionedFn<T4A, C, E4A, Q>,
    ) -> ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4A, E4A, E5, T6, E6>
    where
        T4A: DeserializeOwned + 'static,
        E4A: Display + Debug + Send + Sync + 'static,
    {
        ContractWrapper {
            execute_fn: self.execute_fn,
            instantiate_fn: self.instantiate_fn,
            query_fn: self.query_fn,
            sudo_fn: Some(Box::new(sudo_fn)),
            reply_fn: self.reply_fn,
            migrate_fn: self.migrate_fn,
            checksum: None,

            channel_open_fn: self.channel_open_fn,
            channel_connect_fn: self.channel_connect_fn,
            channel_close_fn: self.channel_close_fn,

            ibc_packet_receive_fn: self.ibc_packet_receive_fn,
            ibc_packet_ack_fn: self.ibc_packet_ack_fn,
            ibc_packet_timeout_fn: self.ibc_packet_timeout_fn,
        }
    }

    /// Populates [ContractWrapper] with contract's `sudo` entry-point and `Empty` as a custom message.
    pub fn with_sudo_empty<T4A, E4A>(
        self,
        sudo_fn: PermissionedFn<T4A, Empty, E4A, Empty>,
    ) -> ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4A, E4A, E5, T6, E6>
    where
        T4A: DeserializeOwned + 'static,
        E4A: Display + Debug + Send + Sync + 'static,
    {
        ContractWrapper {
            execute_fn: self.execute_fn,
            instantiate_fn: self.instantiate_fn,
            query_fn: self.query_fn,
            sudo_fn: Some(customize_permissioned_fn(sudo_fn)),
            reply_fn: self.reply_fn,
            migrate_fn: self.migrate_fn,
            checksum: None,

            channel_open_fn: self.channel_open_fn,
            channel_connect_fn: self.channel_connect_fn,
            channel_close_fn: self.channel_close_fn,

            ibc_packet_receive_fn: self.ibc_packet_receive_fn,
            ibc_packet_ack_fn: self.ibc_packet_ack_fn,
            ibc_packet_timeout_fn: self.ibc_packet_timeout_fn,
        }
    }

    /// Populates [ContractWrapper] with contract's `reply` entry-point and custom message type.
    pub fn with_reply<E5A>(
        self,
        reply_fn: ReplyFn<C, E5A, Q>,
    ) -> ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5A, T6, E6>
    where
        E5A: Display + Debug + Send + Sync + 'static,
    {
        ContractWrapper {
            execute_fn: self.execute_fn,
            instantiate_fn: self.instantiate_fn,
            query_fn: self.query_fn,
            sudo_fn: self.sudo_fn,
            reply_fn: Some(Box::new(reply_fn)),
            migrate_fn: self.migrate_fn,
            checksum: None,

            channel_open_fn: self.channel_open_fn,
            channel_connect_fn: self.channel_connect_fn,
            channel_close_fn: self.channel_close_fn,

            ibc_packet_receive_fn: self.ibc_packet_receive_fn,
            ibc_packet_ack_fn: self.ibc_packet_ack_fn,
            ibc_packet_timeout_fn: self.ibc_packet_timeout_fn,
        }
    }

    /// Populates [ContractWrapper] with contract's `reply` entry-point and `Empty` as a custom message.
    pub fn with_reply_empty<E5A>(
        self,
        reply_fn: ReplyFn<Empty, E5A, Empty>,
    ) -> ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5A, T6, E6>
    where
        E5A: Display + Debug + Send + Sync + 'static,
    {
        ContractWrapper {
            execute_fn: self.execute_fn,
            instantiate_fn: self.instantiate_fn,
            query_fn: self.query_fn,
            sudo_fn: self.sudo_fn,
            reply_fn: Some(customize_permissioned_fn(reply_fn)),
            migrate_fn: self.migrate_fn,
            checksum: None,

            channel_open_fn: self.channel_open_fn,
            channel_connect_fn: self.channel_connect_fn,
            channel_close_fn: self.channel_close_fn,

            ibc_packet_receive_fn: self.ibc_packet_receive_fn,
            ibc_packet_ack_fn: self.ibc_packet_ack_fn,
            ibc_packet_timeout_fn: self.ibc_packet_timeout_fn,
        }
    }

    /// Populates [ContractWrapper] with contract's `migrate` entry-point and custom message type.
    pub fn with_migrate<T6A, E6A>(
        self,
        migrate_fn: PermissionedFn<T6A, C, E6A, Q>,
    ) -> ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5, T6A, E6A>
    where
        T6A: DeserializeOwned + 'static,
        E6A: Display + Debug + Send + Sync + 'static,
    {
        ContractWrapper {
            execute_fn: self.execute_fn,
            instantiate_fn: self.instantiate_fn,
            query_fn: self.query_fn,
            sudo_fn: self.sudo_fn,
            reply_fn: self.reply_fn,
            migrate_fn: Some(Box::new(migrate_fn)),
            checksum: None,

            channel_open_fn: self.channel_open_fn,
            channel_connect_fn: self.channel_connect_fn,
            channel_close_fn: self.channel_close_fn,

            ibc_packet_receive_fn: self.ibc_packet_receive_fn,
            ibc_packet_ack_fn: self.ibc_packet_ack_fn,
            ibc_packet_timeout_fn: self.ibc_packet_timeout_fn,
        }
    }

    /// Populates [ContractWrapper] with contract's `migrate` entry-point and `Empty` as a custom message.
    pub fn with_migrate_empty<T6A, E6A>(
        self,
        migrate_fn: PermissionedFn<T6A, Empty, E6A, Empty>,
    ) -> ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5, T6A, E6A>
    where
        T6A: DeserializeOwned + 'static,
        E6A: Display + Debug + Send + Sync + 'static,
    {
        ContractWrapper {
            execute_fn: self.execute_fn,
            instantiate_fn: self.instantiate_fn,
            query_fn: self.query_fn,
            sudo_fn: self.sudo_fn,
            reply_fn: self.reply_fn,
            migrate_fn: Some(customize_permissioned_fn(migrate_fn)),
            checksum: None,

            channel_open_fn: self.channel_open_fn,
            channel_connect_fn: self.channel_connect_fn,
            channel_close_fn: self.channel_close_fn,

            ibc_packet_receive_fn: self.ibc_packet_receive_fn,
            ibc_packet_ack_fn: self.ibc_packet_ack_fn,
            ibc_packet_timeout_fn: self.ibc_packet_timeout_fn,
        }
    }

    /// Populates [ContractWrapper] with the provided checksum of the contract's Wasm blob.
    pub fn with_checksum(mut self, checksum: Checksum) -> Self {
        self.checksum = Some(checksum);
        self
    }

    /// Adding IBC endpoint capabilities
    pub fn with_ibc<E7A, E8A, E9A, E10A, E11A, E12A>(
        self,
        channel_open_fn: IbcFn<IbcChannelOpenMsg, IbcChannelOpenResponse, E7A, Q>,
        channel_connect_fn: IbcFn<IbcChannelConnectMsg, IbcBasicResponse<C>, E8A, Q>,
        channel_close_fn: IbcFn<IbcChannelCloseMsg, IbcBasicResponse<C>, E9A, Q>,

        ibc_packet_receive_fn: IbcFn<IbcPacketReceiveMsg, IbcReceiveResponse<C>, E10A, Q>,
        ibc_packet_ack_fn: IbcFn<IbcPacketAckMsg, IbcBasicResponse<C>, E11A, Q>,
        ibc_packet_timeout_fn: IbcFn<IbcPacketTimeoutMsg, IbcBasicResponse<C>, E12A, Q>,
    ) -> ContractWrapper<
        T1,
        T2,
        T3,
        E1,
        E2,
        E3,
        C,
        Q,
        T4,
        E4,
        E5,
        T6,
        E6,
        E7A,
        E8A,
        E9A,
        E10A,
        E11A,
        E12A,
    >
    where
        E7A: Display + Debug + Send + Sync + 'static,
        E8A: Display + Debug + Send + Sync + 'static,
        E9A: Display + Debug + Send + Sync + 'static,
        E10A: Display + Debug + Send + Sync + 'static,
        E11A: Display + Debug + Send + Sync + 'static,
        E12A: Display + Debug + Send + Sync + 'static,
    {
        ContractWrapper {
            execute_fn: self.execute_fn,
            instantiate_fn: self.instantiate_fn,
            query_fn: self.query_fn,
            sudo_fn: self.sudo_fn,
            reply_fn: self.reply_fn,
            migrate_fn: self.migrate_fn,
            checksum: None,

            channel_open_fn: Some(Box::new(channel_open_fn)),
            channel_connect_fn: Some(Box::new(channel_connect_fn)),
            channel_close_fn: Some(Box::new(channel_close_fn)),

            ibc_packet_receive_fn: Some(Box::new(ibc_packet_receive_fn)),
            ibc_packet_ack_fn: Some(Box::new(ibc_packet_ack_fn)),
            ibc_packet_timeout_fn: Some(Box::new(ibc_packet_timeout_fn)),
        }
    }
}

fn customize_contract_fn<T, C, E, Q>(
    raw_fn: ContractFn<T, Empty, E, Empty>,
) -> ContractClosure<T, C, E, Q>
where
    T: DeserializeOwned + 'static,
    E: Display + Debug + Send + Sync + 'static,
    C: CustomMsg,
    Q: CustomQuery + DeserializeOwned,
{
    Box::new(
        move |mut deps: DepsMut<Q>,
              env: Env,
              info: MessageInfo,
              msg: T|
              -> Result<Response<C>, E> {
            let deps = decustomize_deps_mut(&mut deps);
            raw_fn(deps, env, info, msg).map(customize_response::<C>)
        },
    )
}

fn customize_query_fn<T, E, Q>(raw_fn: QueryFn<T, E, Empty>) -> QueryClosure<T, E, Q>
where
    T: DeserializeOwned + 'static,
    E: Display + Debug + Send + Sync + 'static,
    Q: CustomQuery + DeserializeOwned,
{
    Box::new(
        move |deps: Deps<Q>, env: Env, msg: T| -> Result<Binary, E> {
            let deps = decustomize_deps(&deps);
            raw_fn(deps, env, msg)
        },
    )
}

fn customize_permissioned_fn<T, C, E, Q>(
    raw_fn: PermissionedFn<T, Empty, E, Empty>,
) -> PermissionedClosure<T, C, E, Q>
where
    T: DeserializeOwned + 'static,
    E: Display + Debug + Send + Sync + 'static,
    C: CustomMsg,
    Q: CustomQuery + DeserializeOwned,
{
    Box::new(
        move |mut deps: DepsMut<Q>, env: Env, msg: T| -> Result<Response<C>, E> {
            let deps = decustomize_deps_mut(&mut deps);
            raw_fn(deps, env, msg).map(customize_response::<C>)
        },
    )
}

fn decustomize_deps_mut<'a, Q>(deps: &'a mut DepsMut<Q>) -> DepsMut<'a, Empty>
where
    Q: CustomQuery + DeserializeOwned,
{
    DepsMut {
        storage: deps.storage,
        api: deps.api,
        querier: QuerierWrapper::new(deps.querier.deref()),
    }
}

fn decustomize_deps<'a, Q>(deps: &'a Deps<'a, Q>) -> Deps<'a, Empty>
where
    Q: CustomQuery + DeserializeOwned,
{
    Deps {
        storage: deps.storage,
        api: deps.api,
        querier: QuerierWrapper::new(deps.querier.deref()),
    }
}

fn customize_response<C>(resp: Response<Empty>) -> Response<C>
where
    C: CustomMsg,
{
    let mut customized_resp = Response::<C>::new()
        .add_submessages(resp.messages.into_iter().map(customize_msg::<C>))
        .add_events(resp.events)
        .add_attributes(resp.attributes);
    customized_resp.data = resp.data;
    customized_resp
}

fn customize_msg<C>(msg: SubMsg<Empty>) -> SubMsg<C>
where
    C: CustomMsg,
{
    SubMsg {
        id: msg.id,
        payload: Binary::default(),
        msg: match msg.msg {
            CosmosMsg::Wasm(wasm) => CosmosMsg::Wasm(wasm),
            CosmosMsg::Bank(bank) => CosmosMsg::Bank(bank),
            CosmosMsg::Staking(staking) => CosmosMsg::Staking(staking),
            CosmosMsg::Distribution(distribution) => CosmosMsg::Distribution(distribution),
            CosmosMsg::Custom(_) => unreachable!(),
            CosmosMsg::Ibc(ibc) => CosmosMsg::Ibc(ibc),
            #[cfg(feature = "cosmwasm_2_0")]
            CosmosMsg::Any(any) => CosmosMsg::Any(any),
            _ => panic!("unknown message variant {:?}", msg),
        },
        gas_limit: msg.gas_limit,
        reply_on: msg.reply_on,
    }
}

impl<T1, T2, T3, E1, E2, E3, C, T4, E4, E5, T6, E6, E7, E8, E9, E10, E11, E12, Q> Contract<C, Q>
    for ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5, T6, E6, E7, E8, E9, E10, E11, E12>
where
    T1: DeserializeOwned, // Type of message passed to `execute` entry-point.
    T2: DeserializeOwned, // Type of message passed to `instantiate` entry-point.
    T3: DeserializeOwned, // Type of message passed to `query` entry-point.
    T4: DeserializeOwned, // Type of message passed to `sudo` entry-point.
    T6: DeserializeOwned, // Type of message passed to `migrate` entry-point.
    E1: Display + Debug + Send + Sync + 'static, // Type of error returned from `execute` entry-point.
    E2: Display + Debug + Send + Sync + 'static, // Type of error returned from `instantiate` entry-point.
    E3: Display + Debug + Send + Sync + 'static, // Type of error returned from `query` entry-point.
    E4: Display + Debug + Send + Sync + 'static, // Type of error returned from `sudo` entry-point.
    E5: Display + Debug + Send + Sync + 'static, // Type of error returned from `reply` entry-point.
    E6: Display + Debug + Send + Sync + 'static, // Type of error returned from `migrate` entry-point.
    E7: Display + Debug + Send + Sync + 'static, // Type of error returned from `channel_open` entry-point.
    E8: Display + Debug + Send + Sync + 'static, // Type of error returned from `channel_connect` entry-point.
    E9: Display + Debug + Send + Sync + 'static, // Type of error returned from `channel_close` entry-point.
    E10: Display + Debug + Send + Sync + 'static, // Type of error returned from `ibc_packet_receive` entry-point.
    E11: Display + Debug + Send + Sync + 'static, // Type of error returned from `ibc_packet_ack` entry-point.
    E12: Display + Debug + Send + Sync + 'static, // Type of error returned from `ibc_packet_timeout` entry-point.
    C: CustomMsg, // Type of custom message returned from all entry-points except `query`.
    Q: CustomQuery + DeserializeOwned, // Type of custom query in querier passed as deps/deps_mut to all entry-points.
{
    /// Calls [execute] on wrapped [Contract] trait implementor.
    ///
    /// [execute]: Contract::execute
    fn execute(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        msg: Vec<u8>,
    ) -> AnyResult<Response<C>> {
        let msg: T1 = from_json(msg)?;
        (self.execute_fn)(deps, env, info, msg).map_err(|err: E1| anyhow!(err))
    }

    /// Calls [instantiate] on wrapped [Contract] trait implementor.
    ///
    /// [instantiate]: Contract::instantiate
    fn instantiate(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        msg: Vec<u8>,
    ) -> AnyResult<Response<C>> {
        let msg: T2 = from_json(msg)?;
        (self.instantiate_fn)(deps, env, info, msg).map_err(|err: E2| anyhow!(err))
    }

    /// Calls [query] on wrapped [Contract] trait implementor.
    ///
    /// [query]: Contract::query
    fn query(&self, deps: Deps<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Binary> {
        let msg: T3 = from_json(msg)?;
        (self.query_fn)(deps, env, msg).map_err(|err: E3| anyhow!(err))
    }

    /// Calls [sudo] on wrapped [Contract] trait implementor.
    /// Returns an error when the contract does not implement [sudo].
    ///
    /// [sudo]: Contract::sudo
    fn sudo(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>> {
        let msg: T4 = from_json(msg)?;
        match &self.sudo_fn {
            Some(sudo) => sudo(deps, env, msg).map_err(|err: E4| anyhow!(err)),
            None => bail!("sudo is not implemented for contract"),
        }
    }

    /// Calls [reply] on wrapped [Contract] trait implementor.
    /// Returns an error when the contract does not implement [reply].
    ///
    /// [reply]: Contract::reply
    fn reply(&self, deps: DepsMut<Q>, env: Env, reply_data: Reply) -> AnyResult<Response<C>> {
        let msg: Reply = reply_data;
        match &self.reply_fn {
            Some(reply) => reply(deps, env, msg).map_err(|err: E5| anyhow!(err)),
            None => bail!("reply is not implemented for contract"),
        }
    }

    /// Calls [migrate] on wrapped [Contract] trait implementor.
    /// Returns an error when the contract does not implement [migrate].
    ///
    /// [migrate]: Contract::migrate
    fn migrate(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>> {
        let msg: T6 = from_json(msg)?;
        match &self.migrate_fn {
            Some(migrate) => migrate(deps, env, msg).map_err(|err: E6| anyhow!(err)),
            None => bail!("migrate is not implemented for contract"),
        }
    }

    /// Returns the provided checksum of the contract's Wasm blob.
    fn checksum(&self) -> Option<Checksum> {
        self.checksum
    }

    fn ibc_channel_open(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcChannelOpenMsg,
    ) -> AnyResult<IbcChannelOpenResponse> {
        match &self.channel_open_fn {
            Some(channel_open) => channel_open(deps, env, msg).map_err(|err| anyhow!(err)),
            None => bail!("channel open not implemented for contract"),
        }
    }
    fn ibc_channel_connect(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcChannelConnectMsg,
    ) -> AnyResult<IbcBasicResponse<C>> {
        match &self.channel_connect_fn {
            Some(channel_connect) => channel_connect(deps, env, msg).map_err(|err| anyhow!(err)),
            None => bail!("channel connect not implemented for contract"),
        }
    }
    fn ibc_channel_close(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcChannelCloseMsg,
    ) -> AnyResult<IbcBasicResponse<C>> {
        match &self.channel_close_fn {
            Some(channel_close) => channel_close(deps, env, msg).map_err(|err| anyhow!(err)),
            None => bail!("channel close not implemented for contract"),
        }
    }

    fn ibc_packet_receive(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcPacketReceiveMsg,
    ) -> AnyResult<IbcReceiveResponse<C>> {
        match &self.ibc_packet_receive_fn {
            Some(packet_receive) => packet_receive(deps, env, msg).map_err(|err| anyhow!(err)),
            None => bail!("packet receive not implemented for contract"),
        }
    }
    fn ibc_packet_acknowledge(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcPacketAckMsg,
    ) -> AnyResult<IbcBasicResponse<C>> {
        match &self.ibc_packet_ack_fn {
            Some(packet_ack) => packet_ack(deps, env, msg).map_err(|err| anyhow!(err)),
            None => bail!("packet ack not implemented for contract"),
        }
    }
    fn ibc_packet_timeout(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcPacketTimeoutMsg,
    ) -> AnyResult<IbcBasicResponse<C>> {
        match &self.ibc_packet_timeout_fn {
            Some(packet_timeout) => packet_timeout(deps, env, msg).map_err(|err| anyhow!(err)),
            None => bail!("packet timeout not implemented for contract"),
        }
    }
}

#[cfg(feature = "mock")]
pub mod cw_multi_test_impl {
    use super::*;
    pub struct BoxedContractWrapper<C, Q>(Box<dyn Contract<C, Q>>);

    impl<C, Q> From<Box<dyn Contract<C, Q>>> for BoxedContractWrapper<C, Q> {
        fn from(value: Box<dyn Contract<C, Q>>) -> Self {
            Self(value)
        }
    }

    impl<C, Q> cw_multi_test::Contract<C, Q> for BoxedContractWrapper<C, Q>
    where
        C: CustomMsg,
        Q: CustomQuery + DeserializeOwned,
    {
        fn execute(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            info: MessageInfo,
            msg: Vec<u8>,
        ) -> AnyResult<Response<C>> {
            Contract::<C, Q>::execute(self.0.as_ref(), deps, env, info, msg)
        }

        fn instantiate(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            info: MessageInfo,
            msg: Vec<u8>,
        ) -> AnyResult<Response<C>> {
            Contract::<C, Q>::instantiate(self.0.as_ref(), deps, env, info, msg)
        }

        fn query(&self, deps: Deps<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Binary> {
            Contract::<C, Q>::query(self.0.as_ref(), deps, env, msg)
        }

        fn sudo(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>> {
            Contract::<C, Q>::sudo(self.0.as_ref(), deps, env, msg)
        }

        fn reply(&self, deps: DepsMut<Q>, env: Env, reply_data: Reply) -> AnyResult<Response<C>> {
            Contract::<C, Q>::reply(self.0.as_ref(), deps, env, reply_data)
        }

        fn migrate(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>> {
            Contract::<C, Q>::migrate(self.0.as_ref(), deps, env, msg)
        }

        fn checksum(&self) -> Option<Checksum> {
            Contract::<C, Q>::checksum(self.0.as_ref())
        }

        fn ibc_channel_open(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcChannelOpenMsg,
        ) -> AnyResult<IbcChannelOpenResponse> {
            Contract::<C, Q>::ibc_channel_open(self.0.as_ref(), deps, env, msg)
        }
        fn ibc_channel_connect(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcChannelConnectMsg,
        ) -> AnyResult<IbcBasicResponse<C>> {
            Contract::<C, Q>::ibc_channel_connect(self.0.as_ref(), deps, env, msg)
        }
        fn ibc_channel_close(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcChannelCloseMsg,
        ) -> AnyResult<IbcBasicResponse<C>> {
            Contract::<C, Q>::ibc_channel_close(self.0.as_ref(), deps, env, msg)
        }

        fn ibc_packet_receive(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcPacketReceiveMsg,
        ) -> AnyResult<IbcReceiveResponse<C>> {
            Contract::<C, Q>::ibc_packet_receive(self.0.as_ref(), deps, env, msg)
        }
        fn ibc_packet_acknowledge(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcPacketAckMsg,
        ) -> AnyResult<IbcBasicResponse<C>> {
            Contract::<C, Q>::ibc_packet_acknowledge(self.0.as_ref(), deps, env, msg)
        }
        fn ibc_packet_timeout(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcPacketTimeoutMsg,
        ) -> AnyResult<IbcBasicResponse<C>> {
            Contract::<C, Q>::ibc_packet_timeout(self.0.as_ref(), deps, env, msg)
        }
    }

    impl<T1, T2, T3, E1, E2, E3, C, T4, E4, E5, T6, E6, E7, E8, E9, E10, E11, E12, Q> Contract<C, Q>
        for cw_multi_test::ContractWrapper<
            T1,
            T2,
            T3,
            E1,
            E2,
            E3,
            C,
            Q,
            T4,
            E4,
            E5,
            T6,
            E6,
            E7,
            E8,
            E9,
            E10,
            E11,
            E12,
        >
    where
        T1: DeserializeOwned, // Type of message passed to `execute` entry-point.
        T2: DeserializeOwned, // Type of message passed to `instantiate` entry-point.
        T3: DeserializeOwned, // Type of message passed to `query` entry-point.
        T4: DeserializeOwned, // Type of message passed to `sudo` entry-point.
        T6: DeserializeOwned, // Type of message passed to `migrate` entry-point.
        E1: Display + Debug + Send + Sync + 'static, // Type of error returned from `execute` entry-point.
        E2: Display + Debug + Send + Sync + 'static, // Type of error returned from `instantiate` entry-point.
        E3: Display + Debug + Send + Sync + 'static, // Type of error returned from `query` entry-point.
        E4: Display + Debug + Send + Sync + 'static, // Type of error returned from `sudo` entry-point.
        E5: Display + Debug + Send + Sync + 'static, // Type of error returned from `reply` entry-point.
        E6: Display + Debug + Send + Sync + 'static, // Type of error returned from `migrate` entry-point.
        E7: Display + Debug + Send + Sync + 'static, // Type of error returned from `channel_open` entry-point.
        E8: Display + Debug + Send + Sync + 'static, // Type of error returned from `channel_connect` entry-point.
        E9: Display + Debug + Send + Sync + 'static, // Type of error returned from `channel_close` entry-point.
        E10: Display + Debug + Send + Sync + 'static, // Type of error returned from `ibc_packet_receive` entry-point.
        E11: Display + Debug + Send + Sync + 'static, // Type of error returned from `ibc_packet_ack` entry-point.
        E12: Display + Debug + Send + Sync + 'static, // Type of error returned from `ibc_packet_timeout` entry-point.
        C: CustomMsg, // Type of custom message returned from all entry-points except `query`.
        Q: CustomQuery + DeserializeOwned, // Type of custom query in querier passed as deps/deps_mut to all entry-points.
    {
        fn execute(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            info: MessageInfo,
            msg: Vec<u8>,
        ) -> AnyResult<Response<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::execute(self, deps, env, info, msg)
        }

        fn instantiate(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            info: MessageInfo,
            msg: Vec<u8>,
        ) -> AnyResult<Response<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::instantiate(self, deps, env, info, msg)
        }

        fn query(&self, deps: Deps<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Binary> {
            <Self as cw_multi_test::Contract<C, Q>>::query(self, deps, env, msg)
        }

        fn sudo(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::sudo(self, deps, env, msg)
        }

        fn reply(&self, deps: DepsMut<Q>, env: Env, reply_data: Reply) -> AnyResult<Response<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::reply(self, deps, env, reply_data)
        }

        fn migrate(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::migrate(self, deps, env, msg)
        }

        fn checksum(&self) -> Option<Checksum> {
            <Self as cw_multi_test::Contract<C, Q>>::checksum(self)
        }

        fn ibc_channel_open(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcChannelOpenMsg,
        ) -> AnyResult<IbcChannelOpenResponse> {
            <Self as cw_multi_test::Contract<C, Q>>::ibc_channel_open(self, deps, env, msg)
        }
        fn ibc_channel_connect(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcChannelConnectMsg,
        ) -> AnyResult<IbcBasicResponse<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::ibc_channel_connect(self, deps, env, msg)
        }
        fn ibc_channel_close(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcChannelCloseMsg,
        ) -> AnyResult<IbcBasicResponse<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::ibc_channel_close(self, deps, env, msg)
        }

        fn ibc_packet_receive(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcPacketReceiveMsg,
        ) -> AnyResult<IbcReceiveResponse<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::ibc_packet_receive(self, deps, env, msg)
        }
        fn ibc_packet_acknowledge(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcPacketAckMsg,
        ) -> AnyResult<IbcBasicResponse<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::ibc_packet_acknowledge(self, deps, env, msg)
        }
        fn ibc_packet_timeout(
            &self,
            deps: DepsMut<Q>,
            env: Env,
            msg: IbcPacketTimeoutMsg,
        ) -> AnyResult<IbcBasicResponse<C>> {
            <Self as cw_multi_test::Contract<C, Q>>::ibc_packet_timeout(self, deps, env, msg)
        }
    }
}
