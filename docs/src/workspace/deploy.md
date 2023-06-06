# Interface Collection

Follow the same process as for a single contract to generate the interfaces for your contract.  

Now that you have your contract interfaces you can export a comprehensive wrapper of your application that can easily be used by others.  

The idea is simple. If you can provide an easy way for others to deploy your contracts/application to their environments, then you're making it extremely easy for them to use and build on your application.  

As an example we will create a deployment for the [Abstract smart-contract framework](https://abstract.money/).

## The deployment

The deployment can be represented by a struct containing all the contracts that are uploaded and instantiated when the protocol is deployed.

```rust,ignore


// Our Abstract deployment
pub struct Abstract<Chain: CwEnv> {
    pub ans_host: AnsHost<Chain>,
    pub version_control: VersionControl<Chain>,
}
```

### Implementing `Deploy`

Now we can implement the `cw_orch::Deploy` trait for the `Abstract` struct.

```rust,ignore
# use cw_orch::prelude::*;
# use cw_orch::error::CwOrchError;
# use cw_multi_test::ContractWrapper;
# use abstract_interface::{AnsHost, VersionControl};
# pub struct Abstract<Chain: CwEnv> {
#   pub ans_host: AnsHost<Chain>,
#   pub version_control: VersionControl<Chain>,
# }

impl<Chain: CwEnv> cw_orch::Deploy<Chain> for Abstract<Chain> {
    // We don't have a custom error type
    type Error = CwOrchError;
    type DeployData = semver::Version;

    fn store_on(chain: Chain) -> Result<Self, Self::Error> {
        // "abstract" is a reserved keyword in rust!
        let mut abstrct = Abstract::new(chain);

        // Upload the contracts to the chain
        abstrct.ans_host.upload()?;
        abstrct.version_control.upload()?;

        Ok(abstrct)
    }

    fn deploy_on(chain: Chain, version: semver::Version) -> Result<Self, CwOrchError> {        
        // ########### Upload ##############
        let abstrct = Self::store_on(chain)?;

        // ########### Instantiate ##############
        abstrct.ans_host.instantiate(
            &abstract_core::ans_host::InstantiateMsg {},
            Some(sender),
            None,
        )?;

        abstrct.version_control.instantiate(
            &abstract_core::version_control::InstantiateMsg {},
            Some(sender),
            None,
        )?;

        // ... 

        Ok(abstrct)
    }

    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        let abstrct = Self::new(chain);
        Ok(abstrct)
    }
}
```

Now `Abstract` is an application that can be deployed to a mock and real environment with **one** line of code.

```rust
fn setup_test(mock: Mock) -> Result<(), CwOrchError> {
    let version = "1.0.0".parse().unwrap();
    // Deploy abstract
    let abstract_ = Abstract::deploy_on(mock.clone(), version)?;
}
```

And then when setting up your own deployment you can load these applications to access their contracts (for accessing configuration, addresses, ...)

```rust
impl<Chain: CwEnv> cw_orch::Deploy<Chain> for MyApplication<Chain> {
    /// ...
    fn deploy_on(chain: Chain, _data: Empty) -> Result<Self, CwOrchError> {

        let abstract_: Abstract = Abstract::load_from(chain)?;

        /// ... do stuff with Abstract
    }
}
```
