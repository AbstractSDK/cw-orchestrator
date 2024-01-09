const CONNECTIVITY_LOGS: &str = "Connectivity";
const QUERY_LOGS: &str = "Query";
const CONTRACT_LOGS: &str = "Contract";
const TRANSACTION_LOGS: &str = "Transaction";
const TX_RETRY_LOGS: &str = "Transaction Retry";
const LOCAL_LOGS: &str = "Local";

fn format_aligned(a: &str) -> String {
    format!("{:>17}", a)
}

pub fn connectivity_target() -> String {
    format_aligned(CONNECTIVITY_LOGS)
}
pub fn query_target() -> String {
    format_aligned(QUERY_LOGS)
}
pub fn contract_target() -> String {
    format_aligned(CONTRACT_LOGS)
}
pub fn transaction_target() -> String {
    format_aligned(TRANSACTION_LOGS)
}
pub fn tx_retry_target() -> String {
    format_aligned(TX_RETRY_LOGS)
}
pub fn local_target() -> String {
    format_aligned(LOCAL_LOGS)
}
