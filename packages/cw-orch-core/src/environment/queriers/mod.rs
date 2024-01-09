use self::{node::NodeQuerierGetter, bank::BankQuerierGetter};

pub mod bank;
pub mod node;

pub trait AllQueriers: NodeQuerierGetter + BankQuerierGetter {}

impl<T> AllQueriers for T where T: NodeQuerierGetter + BankQuerierGetter {}
