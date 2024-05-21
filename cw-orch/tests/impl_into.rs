// ANCHOR: impl_into
use cw_orch::interface;
use cw_orch::prelude::*;

// An execute message that is generic.
#[cosmwasm_schema::cw_serde]
pub enum GenericExecuteMsg<T> {
    Generic(T),
    Nested(NestedMessageType),
}

// This is the message that will be used on our contract
type ExecuteMsg = GenericExecuteMsg<Foo>;
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum Foo {
    Bar { a: String },
}

impl From<Foo> for ExecuteMsg {
    fn from(msg: Foo) -> Self {
        ExecuteMsg::Generic(msg)
    }
}

#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum NestedMessageType {
    Test { b: u64 },
}

impl From<NestedMessageType> for ExecuteMsg {
    fn from(msg: NestedMessageType) -> Self {
        ExecuteMsg::Nested(msg)
    }
}

#[interface(Empty, ExecuteMsg, Empty, Empty)]
struct Example<Chain>;

impl<Chain: CwEnv> Example<Chain> {
    pub fn test_macro(&self) {
        #[allow(deprecated)]
        // function `bar` is available because of the `impl_into` attribute!
        self.bar("hello".to_string()).unwrap_err();

        #[allow(deprecated)]
        // function `test` is available because of the `impl_into` attribute!
        self.test(65).unwrap_err();
    }
}

// ANCHOR_END: impl_into
