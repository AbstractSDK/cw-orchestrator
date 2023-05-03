use cosmwasm_std::{CustomQuery, CustomMsg, Addr};
use cw_multi_test::ContractWrapper;
use cw_orch::{Mock};

mod common;

const SENDER: &str = "sender";

#[test]
    fn custom_app() {
        #[cosmwasm_schema::cw_serde]
        struct Foo {}

        impl CustomQuery for Foo {}
        impl CustomMsg for Foo {}

        let sender = &Addr::unchecked(SENDER);
        let app = Mock::<_,Foo>::new(sender).unwrap();
        let cw20 = common::contract::Cw20::new(app.clone());
        
        app.upload_custom(Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            )
            .with_migrate_empty( cw20_base::contract::migrate),
        )).unwrap();
}