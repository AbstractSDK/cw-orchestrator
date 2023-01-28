# Integration

Given your contract interfaces you can export a comprehensive deployment of your application to be used by others.  

The idea is simple. If you can provide an easy way for others to instantiate your contracts in their environments then you're making it extremely pleasant for them to use your application as a dependency in their application.  

As an example we will create a deployment for the [Abstract smart-contract framework](https://abstract.money/).

## The deployment

The deployment can be represented by a struct containing all the contracts that are instantiated when the protocol is deployed.

```rust
// Our Abstract deployment
pub struct Abstract<Chain: BootEnvironment> {
    pub version: Version,
    pub ans_host: AnsHost<Chain>,
    pub version_control: VersionControl<Chain>,
}
```

### Implementing `Deploy`

Now we can implement the `boot_core::Deploy` trait for the `Abstract` struct.

```rust
impl<Chain: BootEnvironment> boot_core::Deploy<Chain> for Abstract<Chain> {
    // We don't have a custom error type
    type Error = BootError;
    type DeployData = semver::Version;

    fn deploy_on(chain: Chain, version: semver::Version) -> Result<Self, BootError> {
        let mut ans_host = AnsHost::new(ANS_HOST, chain.clone());
        let mut version_control = VersionControl::new(VERSION_CONTROL, chain.clone());

        // Only include mock when `integration` flag is set
        if cfg!(feature = "integration") {
            ans_host
                .as_instance_mut()
                .set_mock(Box::new(ContractWrapper::new_with_empty(
                    ::ans_host::contract::execute,
                    ::ans_host::contract::instantiate,
                    ::ans_host::contract::query,
                )));

            version_control.as_instance_mut().set_mock(Box::new(
                cw_multi_test::ContractWrapper::new_with_empty(
                    ::version_control::contract::execute,
                    ::version_control::contract::instantiate,
                    ::version_control::contract::query,
                ),
            ));
        }

        // ########### Upload ##############

        ans_host.upload()?;
        version_control.upload()?;

        // ########### Instantiate ##############

        ans_host.instantiate(
            &abstract_os::ans_host::InstantiateMsg {},
            Some(sender),
            None,
        )?;

        version_control.instantiate(
            &abstract_os::version_control::InstantiateMsg {},
            Some(sender),
            None,
        )?;

        // ... 

        let deployment = Abstract {
            version,
            ans_host,
            version_control,
        };

        Ok(deployment)
    }

    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        // Construct the same structs as above (same names)
        let ans_host = AnsHost::new(ANS_HOST, chain.clone());
        let version_control = VersionControl::new(VERSION_CONTROL, chain.clone());
        let version = env!("CARGO_PKG_VERSION").parse().unwrap();
        Ok(Self {
            chain,
            version,
            ans_host,
            version_control,
            os_factory,
            module_factory,
        })
    }
}
```

Now `Abstract` is an application that can be deployed to a mock and real environment with **one** line of code.

```rust
fn setup_test(mock: Mock) -> Result<(), BootError> {
    let version = "1.0.0".parse().unwrap();
    // Deploy abstract
    let abstract_ = Abstract::deploy_on(mock.clone(), version)?;
}
```

And then when setting up your own deployment you can load these applications to access their contracts (for accessing configuration, addresses, ...)

```rust
impl<Chain: BootEnvironment> boot_core::Deploy<Chain> for MyApplication<Chain> {
    /// ...
    fn deploy_on(chain: Chain, _data: Empty) -> Result<Self, BootError> {

        let abstract_: Abstract = Abstract::load_from(chain)?;

        /// ... do stuff with Abstract
    }
}
```
