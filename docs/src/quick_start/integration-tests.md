# Integration Tests

To get started writing integration tests with cw-orchestrator, first add `cw-orchestrator` to your dependencies.

```bash
cargo add --dev cw-orch
```

The following assumes that you have written your interfaces as described in [Interfaces](./quick_start/interfaces.md).

## Test Setup

```rust
// contracts/my-contract/tests/integration_tests.rs  
use interfaces::MyContract;
use cw_orch::*;  
use semver::Version;
use my_contract::contract;
use my_contract::{ExecuteMsg};
  
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

    // "upload" the contract to the test environment
    // stores the code_id at the key "testing:contract".
    my_contract.upload(); 
  
    Ok(my_contract)  
}  
  
#[test]  
fn test_something() -> anyhow::Result<()> {  
  let my_contract = setup()?;  

  my_contract.instantiate(...)?;

  // execute on the contract
  my_contract.execute(&ExecuteMsg::MyContractCall {
    something: Some(Addr::unchecked("arg"))
  });
}
```
