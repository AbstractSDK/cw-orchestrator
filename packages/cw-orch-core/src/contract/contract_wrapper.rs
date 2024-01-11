use anyhow::{anyhow, bail, Error as AnyError, Result as AnyResult};
use cosmwasm_std::{
    from_json, Binary, CustomQuery, Deps, DepsMut, Empty, Env, MessageInfo, Reply, Response,
};
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fmt::{Debug, Display};

use cosmwasm_std::{
    IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg,
    IbcChannelOpenResponse, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse,
};
use schemars::JsonSchema;

/// Interface to call into a `Contract`.
pub trait MockContract<T, Q = Empty>
where
    T: Clone + Debug + PartialEq + JsonSchema,
    Q: CustomQuery,
{
    fn execute(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        msg: Vec<u8>,
    ) -> AnyResult<Response<T>>;

    fn instantiate(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        msg: Vec<u8>,
    ) -> AnyResult<Response<T>>;

    fn query(&self, deps: Deps<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Binary>;

    fn sudo(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<T>>;

    fn reply(&self, deps: DepsMut<Q>, env: Env, msg: Reply) -> AnyResult<Response<T>>;

    fn migrate(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<T>>;

    #[allow(unused)]
    fn ibc_channel_open(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcChannelOpenMsg,
    ) -> AnyResult<IbcChannelOpenResponse> {
        bail!("No Ibc capabilities on this contract")
    }

    #[allow(unused)]
    fn ibc_channel_connect(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcChannelConnectMsg,
    ) -> AnyResult<IbcBasicResponse<T>> {
        bail!("No Ibc capabilities on this contract")
    }

    #[allow(unused)]
    fn ibc_channel_close(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcChannelCloseMsg,
    ) -> AnyResult<IbcBasicResponse<T>> {
        bail!("No Ibc capabilities on this contract")
    }

    #[allow(unused)]
    fn ibc_packet_receive(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcPacketReceiveMsg,
    ) -> AnyResult<IbcReceiveResponse<T>> {
        bail!("No Ibc capabilities on this contract")
    }

    #[allow(unused)]
    fn ibc_packet_acknowledge(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcPacketAckMsg,
    ) -> AnyResult<IbcBasicResponse<T>> {
        bail!("No Ibc capabilities on this contract")
    }

    #[allow(unused)]
    fn ibc_packet_timeout(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        msg: IbcPacketTimeoutMsg,
    ) -> AnyResult<IbcBasicResponse<T>> {
        bail!("No Ibc capabilities on this contract")
    }
}

type ContractFn<T, C, E, Q> =
    fn(deps: DepsMut<Q>, env: Env, info: MessageInfo, msg: T) -> Result<Response<C>, E>;
type PermissionedFn<T, C, E, Q> = fn(deps: DepsMut<Q>, env: Env, msg: T) -> Result<Response<C>, E>;
type ReplyFn<C, E, Q> = fn(deps: DepsMut<Q>, env: Env, msg: Reply) -> Result<Response<C>, E>;
type QueryFn<T, E, Q> = fn(deps: Deps<Q>, env: Env, msg: T) -> Result<Binary, E>;

type IbcFn<T, R, E, Q> = fn(deps: DepsMut<Q>, env: Env, msg: T) -> Result<R, E>;

type ContractClosure<T, C, E, Q> =
    Box<dyn Fn(DepsMut<Q>, Env, MessageInfo, T) -> Result<Response<C>, E>>;
type PermissionedClosure<T, C, E, Q> = Box<dyn Fn(DepsMut<Q>, Env, T) -> Result<Response<C>, E>>;
type ReplyClosure<C, E, Q> = Box<dyn Fn(DepsMut<Q>, Env, Reply) -> Result<Response<C>, E>>;
type QueryClosure<T, E, Q> = Box<dyn Fn(Deps<Q>, Env, T) -> Result<Binary, E>>;

/// Wraps the exported functions from a contract and provides the normalized format
/// Place T4 and E4 at the end, as we just want default placeholders for most contracts that don't have sudo
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
    T1: DeserializeOwned + Debug,
    T2: DeserializeOwned,
    T3: DeserializeOwned,
    T4: DeserializeOwned,
    T6: DeserializeOwned,
    E1: Display + Debug + Send + Sync + 'static,
    E2: Display + Debug + Send + Sync + 'static,
    E3: Display + Debug + Send + Sync + 'static,
    E4: Display + Debug + Send + Sync + 'static,
    E5: Display + Debug + Send + Sync + 'static,
    E6: Display + Debug + Send + Sync + 'static,
    E7: Display + Debug + Send + Sync + 'static,
    E8: Display + Debug + Send + Sync + 'static,
    E9: Display + Debug + Send + Sync + 'static,
    E10: Display + Debug + Send + Sync + 'static,
    E11: Display + Debug + Send + Sync + 'static,
    E12: Display + Debug + Send + Sync + 'static,
    C: Clone + Debug + PartialEq + JsonSchema,
    Q: CustomQuery + DeserializeOwned + 'static,
{
    pub execute_fn: ContractClosure<T1, C, E1, Q>,
    pub instantiate_fn: ContractClosure<T2, C, E2, Q>,
    pub query_fn: QueryClosure<T3, E3, Q>,
    pub sudo_fn: Option<PermissionedClosure<T4, C, E4, Q>>,
    pub reply_fn: Option<ReplyClosure<C, E5, Q>>,
    pub migrate_fn: Option<PermissionedClosure<T6, C, E6, Q>>,

    pub channel_open_fn: Option<IbcFn<IbcChannelOpenMsg, IbcChannelOpenResponse, E7, Q>>,
    pub channel_connect_fn: Option<IbcFn<IbcChannelConnectMsg, IbcBasicResponse<C>, E8, Q>>,
    pub channel_close_fn: Option<IbcFn<IbcChannelCloseMsg, IbcBasicResponse<C>, E9, Q>>,

    pub ibc_packet_receive_fn: Option<IbcFn<IbcPacketReceiveMsg, IbcReceiveResponse<C>, E10, Q>>,
    pub ibc_packet_ack_fn: Option<IbcFn<IbcPacketAckMsg, IbcBasicResponse<C>, E11, Q>>,
    pub ibc_packet_timeout_fn: Option<IbcFn<IbcPacketTimeoutMsg, IbcBasicResponse<C>, E12, Q>>,
}

impl<T1, T2, T3, E1, E2, E3, C, Q> ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q>
where
    T1: DeserializeOwned + Debug + 'static,
    T2: DeserializeOwned + 'static,
    T3: DeserializeOwned + 'static,
    E1: Display + Debug + Send + Sync + 'static,
    E2: Display + Debug + Send + Sync + 'static,
    E3: Display + Debug + Send + Sync + 'static,
    C: Clone + Debug + PartialEq + JsonSchema + 'static,
    Q: CustomQuery + DeserializeOwned + 'static,
{
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

            channel_open_fn: None,
            channel_connect_fn: None,
            channel_close_fn: None,

            ibc_packet_receive_fn: None,
            ibc_packet_ack_fn: None,
            ibc_packet_timeout_fn: None,
        }
    }
}

impl<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5, T6, E6>
    ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5, T6, E6>
where
    T1: DeserializeOwned + Debug + 'static,
    T2: DeserializeOwned + 'static,
    T3: DeserializeOwned + 'static,
    T4: DeserializeOwned + 'static,
    T6: DeserializeOwned + 'static,
    E1: Display + Debug + Send + Sync + 'static,
    E2: Display + Debug + Send + Sync + 'static,
    E3: Display + Debug + Send + Sync + 'static,
    E4: Display + Debug + Send + Sync + 'static,
    E5: Display + Debug + Send + Sync + 'static,
    E6: Display + Debug + Send + Sync + 'static,
    C: Clone + Debug + PartialEq + JsonSchema + 'static,
    Q: CustomQuery + DeserializeOwned + 'static,
{
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

            channel_open_fn: self.channel_open_fn,
            channel_connect_fn: self.channel_connect_fn,
            channel_close_fn: self.channel_close_fn,

            ibc_packet_receive_fn: self.ibc_packet_receive_fn,
            ibc_packet_ack_fn: self.ibc_packet_ack_fn,
            ibc_packet_timeout_fn: self.ibc_packet_timeout_fn,
        }
    }

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

            channel_open_fn: self.channel_open_fn,
            channel_connect_fn: self.channel_connect_fn,
            channel_close_fn: self.channel_close_fn,

            ibc_packet_receive_fn: self.ibc_packet_receive_fn,
            ibc_packet_ack_fn: self.ibc_packet_ack_fn,
            ibc_packet_timeout_fn: self.ibc_packet_timeout_fn,
        }
    }

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

            channel_open_fn: self.channel_open_fn,
            channel_connect_fn: self.channel_connect_fn,
            channel_close_fn: self.channel_close_fn,

            ibc_packet_receive_fn: self.ibc_packet_receive_fn,
            ibc_packet_ack_fn: self.ibc_packet_ack_fn,
            ibc_packet_timeout_fn: self.ibc_packet_timeout_fn,
        }
    }

    // Adding IBC endpoint capabilities
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

            channel_open_fn: Some(channel_open_fn),
            channel_connect_fn: Some(channel_connect_fn),
            channel_close_fn: Some(channel_close_fn),

            ibc_packet_receive_fn: Some(ibc_packet_receive_fn),
            ibc_packet_ack_fn: Some(ibc_packet_ack_fn),
            ibc_packet_timeout_fn: Some(ibc_packet_timeout_fn),
        }
    }
}

impl<T1, T2, T3, E1, E2, E3, C, T4, E4, E5, T6, E6, E7, E8, E9, E10, E11, E12, Q> MockContract<C, Q>
    for ContractWrapper<T1, T2, T3, E1, E2, E3, C, Q, T4, E4, E5, T6, E6, E7, E8, E9, E10, E11, E12>
where
    T1: DeserializeOwned + Debug + Clone,
    T2: DeserializeOwned + Debug + Clone,
    T3: DeserializeOwned + Debug + Clone,
    T4: DeserializeOwned,
    T6: DeserializeOwned,
    E1: Display + Debug + Send + Sync + Error + 'static,
    E2: Display + Debug + Send + Sync + Error + 'static,
    E3: Display + Debug + Send + Sync + Error + 'static,
    E4: Display + Debug + Send + Sync + 'static,
    E5: Display + Debug + Send + Sync + 'static,
    E6: Display + Debug + Send + Sync + 'static,
    E7: Display + Debug + Send + Sync + 'static,
    E8: Display + Debug + Send + Sync + 'static,
    E9: Display + Debug + Send + Sync + 'static,
    E10: Display + Debug + Send + Sync + 'static,
    E11: Display + Debug + Send + Sync + 'static,
    E12: Display + Debug + Send + Sync + 'static,
    C: Clone + Debug + PartialEq + JsonSchema,
    Q: CustomQuery + DeserializeOwned,
{
    fn execute(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        msg: Vec<u8>,
    ) -> AnyResult<Response<C>> {
        let msg: T1 = from_json(msg)?;
        (self.execute_fn)(deps, env, info, msg).map_err(|err| anyhow!(err))
    }

    fn instantiate(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        msg: Vec<u8>,
    ) -> AnyResult<Response<C>> {
        let msg: T2 = from_json(msg)?;
        (self.instantiate_fn)(deps, env, info, msg).map_err(|err| anyhow!(err))
    }

    fn query(&self, deps: Deps<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Binary> {
        let msg: T3 = from_json(msg)?;
        (self.query_fn)(deps, env, msg).map_err(|err| anyhow!(err))
    }

    // this returns an error if the contract doesn't implement sudo
    fn sudo(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>> {
        let msg = from_json(msg)?;
        match &self.sudo_fn {
            Some(sudo) => sudo(deps, env, msg).map_err(|err| anyhow!(err)),
            None => bail!("sudo not implemented for contract"),
        }
    }

    // this returns an error if the contract doesn't implement reply
    fn reply(&self, deps: DepsMut<Q>, env: Env, reply_data: Reply) -> AnyResult<Response<C>> {
        match &self.reply_fn {
            Some(reply) => reply(deps, env, reply_data).map_err(|err| anyhow!(err)),
            None => bail!("reply not implemented for contract"),
        }
    }

    // this returns an error if the contract doesn't implement migrate
    fn migrate(&self, deps: DepsMut<Q>, env: Env, msg: Vec<u8>) -> AnyResult<Response<C>> {
        let msg = from_json(msg)?;
        match &self.migrate_fn {
            Some(migrate) => migrate(deps, env, msg).map_err(|err| anyhow!(err)),
            None => bail!("migrate not implemented for contract"),
        }
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
