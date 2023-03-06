use bollard::service::ContainerSummary;

pub const HERMES_ID: &str = "hermes";

pub struct Hermes(pub(crate) ContainerSummary);

impl Hermes {
    pub fn new() {}

    // hermes create channel --channel-version simple-ica-v2 --a-chain juno-1 --b-chain osmosis-2 --a-port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --b-port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --new-client-connection
    pub fn channels() {
        // Command::new("sh").args([""])
    }
}
