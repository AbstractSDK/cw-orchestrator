use bollard::service::ContainerSummary;

pub const HERMES_ID: &str = "hermes";

pub struct Hermes(pub(crate) ContainerSummary);

impl Hermes {
    pub fn new() {}

    pub fn channels() {
        // Command::new("sh").args([""])
    }
}
