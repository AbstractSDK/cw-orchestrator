pub trait StateInterface{
    fn address(&self, key: &str) -> String;
    fn save_address(&self, address: &str);
}