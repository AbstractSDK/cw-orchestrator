use counter_contract::CounterContract;
use cw_orch::{contract::interface_traits::Uploadable, daemon::networks::OSMOSIS_1, mock::Mock};

#[test]
fn checksum() {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    let path = Path::new("../../../artifacts/checksums.txt");
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut found = false;

    for line in lines.map_while(Result::ok) {
        if line.contains("counter_contract.wasm") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                let calculated_hash = CounterContract::<Mock>::wasm(&OSMOSIS_1.into())
                    .checksum()
                    .unwrap();
                assert_eq!(parts[0], calculated_hash.to_string());
                found = true;
                break;
            }
        }
    }

    if !found {
        panic!("Checksum of counter_contract.wasm not found in checksums.txt");
    }
}
