# Integration Tests

To get started writing integration tests with BOOT, first take dependencis on `boot-core` and `cw-multi-test`.

```bash
cargo add --dev boot-core cw-multi-test
```

The following assumes that you have written your interfaces as described in [Interfaces](./quick_start/interfaces.md) Iy you want to define a mock interface explicitly, see below.

## Test Setup

```rust
// contracts/my-contract/tests/integration_tests.rs

use boot_core::networks;  
use interfaces::MyContract;
use boot_core::*;  
use cosmwasm_std::Addr;  
use cw_multi_test::ContractWrapper;
use semver::Version;
use my_contract::contract;
use my_contract::{ExecuteMsg};
  
const TEST_VERSION: Version = Version::new(0, 0, 0);  
  
fn setup() -> anyhow::Result<MyContract<Mock>> {  
    let sender = Addr::unchecked("sender"); 

    // note that we are instantiating the mock environment instead of the daemon_environment
    let (_, mock_chain) = instantiate_default_mock_env(&sender)?;  


    let mut my_contract = MyContract::new("testing:contract", &mock_chain);  

    // entrypoint endpoints for your contract
    my_contract.as_instance_mut().set_mock(Box::new(  
        ContractWrapper::new_with_empty(
            contract::execute,  
            contract::instantiate,  
            contract::query,  
        ),
    ));

    // "upload" the contract to the test environment, storing the code_id
    my_contract.upload(); 
  
    Ok(my_contract)  
}  
  
#[test]  
fn test_something() {  
  let my_contract = setup();  


  // test my contract
  my_contract.execute(ExecuteMsg::MyContractCall {
    something: Some(Addr::unchecked("arg"))
  });
}
```

## Exclusively Mock Interfaces

See [Interfaces](./quick_start/interfaces.md)  for explanations.

To define an interface *exclusively* for use in integration testing, create an interface using the `with_mock` builder method:

```rust
// contracts/my-contract/src/tests/mock-my-contract.rs

use my_contract::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MockMyContract<Chain>;
use cw_multi_test::ContractWrapper;
use crate::contract;
use boot_core::ß*;


impl<Chain: CwEnv> MockMyContract<Chain> {
    /// Construct a new instance of MyContract
    /// * `contract_id` - what your contract should be called in local state (*not* on-chain)
    /// * `chain` - the environment to deploy to
    pub fn new(contract_id: &str, chain: &Chain) -> Self {
        Self(
            Contract::new(contract_id, chain)
                // Mocked entry points allow for use in the mock environment (cw-multi-test)
                .with_mock(Box::new(
                   ContractWrapper::new_with_empty(
                     my_contract::contract::execute,
                     my_contract::contract::instantiate,
                     my_contract::contract::query,
                )))
                ,
        )
    }
}
```
