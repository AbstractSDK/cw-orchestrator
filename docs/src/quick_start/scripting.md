# Writing and Executing Scripts

Now that we have the interface written for our contract, we can start writing scripts to deploy and interact with it.
Setup
Like before, we're going to setup a new folder for our scripts. This time, we'll call it scripts and initialize it as a binary crate:
cargo init --bin scripts
If your cargo project is a workspace, be sure to add scripts to the [workspace].members array at the workspace root.
Your scripts will have basically the same dependencies as your contract interfaces, but with a few additions:
cargo add --path ../interfaces
and also add the anyhow and dotenv crates:
cargo add anyhow dotenv log
Env Configuration
The dotenv crate will allow us to load environment variables from a .env file. This is useful for setting up the chain configuration for your scripts.