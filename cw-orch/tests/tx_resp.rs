use cosmrs::proto::tendermint::abci::{Event, EventAttribute};
use cw_orch::daemon::parse_timestamp;
use cw_orch::daemon::CosmTxResponse;
use cw_orch::daemon::TxResultBlockMsg;
use cw_orch::prelude::IndexResponse;
use speculoos::prelude::*;

use serde_json::Value;

const TEST_TX: &str = r#"{"tx_response":{"height":"4520713","txhash":"B8F9DA7DAB4C7A1A7374B3810A60DE4F2E7E3A9B67A8F54072021096F71F4AB0","codespace":"","code":0,"data":"0A260A242F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E74726163740A260A242F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E74726163740A260A242F636F736D7761736D2E7761736D2E76312E4D736745786563757465436F6E74726163740A1E0A1C2F636F736D6F732E62616E6B2E763162657461312E4D736753656E640A1E0A1C2F636F736D6F732E62616E6B2E763162657461312E4D736753656E64","raw_log":"[{\"events\":[{\"type\":\"coin_received\",\"attributes\":[{\"key\":\"receiver\",\"value\":\"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr\"},{\"key\":\"amount\",\"value\":\"13603122uluna\"},{\"key\":\"receiver\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"17768867ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"},{\"key\":\"receiver\",\"value\":\"terra1ygcvxp9s054q8u2q4hvl52ke393zvgj0sllahlycm4mj8dm96zjsa45rzk\"},{\"key\":\"amount\",\"value\":\"17820ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"}]},{\"type\":\"coin_spent\",\"attributes\":[{\"key\":\"spender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"13603122uluna\"},{\"key\":\"spender\",\"value\":\"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr\"},{\"key\":\"amount\",\"value\":\"17768867ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"},{\"key\":\"spender\",\"value\":\"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr\"},{\"key\":\"amount\",\"value\":\"17820ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"}]},{\"type\":\"execute\",\"attributes\":[{\"key\":\"_contract_address\",\"value\":\"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr\"}]},{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmwasm.wasm.v1.MsgExecuteContract\"},{\"key\":\"module\",\"value\":\"wasm\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"}]},{\"type\":\"transfer\",\"attributes\":[{\"key\":\"recipient\",\"value\":\"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"13603122uluna\"},{\"key\":\"recipient\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"sender\",\"value\":\"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr\"},{\"key\":\"amount\",\"value\":\"17768867ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"},{\"key\":\"recipient\",\"value\":\"terra1ygcvxp9s054q8u2q4hvl52ke393zvgj0sllahlycm4mj8dm96zjsa45rzk\"},{\"key\":\"sender\",\"value\":\"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr\"},{\"key\":\"amount\",\"value\":\"17820ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"}]},{\"type\":\"wasm\",\"attributes\":[{\"key\":\"_contract_address\",\"value\":\"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr\"},{\"key\":\"action\",\"value\":\"swap\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"receiver\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"offer_asset\",\"value\":\"uluna\"},{\"key\":\"ask_asset\",\"value\":\"ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"},{\"key\":\"offer_amount\",\"value\":\"13603122\"},{\"key\":\"return_amount\",\"value\":\"17768867\"},{\"key\":\"tax_amount\",\"value\":\"0\"},{\"key\":\"spread_amount\",\"value\":\"111\"},{\"key\":\"commission_amount\",\"value\":\"53466\"},{\"key\":\"maker_fee_amount\",\"value\":\"17820\"}]}]},{\"msg_index\":1,\"events\":[{\"type\":\"coin_received\",\"attributes\":[{\"key\":\"receiver\",\"value\":\"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6\"},{\"key\":\"amount\",\"value\":\"17768866ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"}]},{\"type\":\"coin_spent\",\"attributes\":[{\"key\":\"spender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"17768866ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"}]},{\"type\":\"execute\",\"attributes\":[{\"key\":\"_contract_address\",\"value\":\"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6\"},{\"key\":\"_contract_address\",\"value\":\"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26\"},{\"key\":\"_contract_address\",\"value\":\"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26\"}]},{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmwasm.wasm.v1.MsgExecuteContract\"},{\"key\":\"module\",\"value\":\"wasm\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"}]},{\"type\":\"transfer\",\"attributes\":[{\"key\":\"recipient\",\"value\":\"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"17768866ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"}]},{\"type\":\"wasm\",\"attributes\":[{\"key\":\"_contract_address\",\"value\":\"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6\"},{\"key\":\"action\",\"value\":\"swap\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"receiver\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"offer_asset\",\"value\":\"ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4\"},{\"key\":\"ask_asset\",\"value\":\"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26\"},{\"key\":\"offer_amount\",\"value\":\"17768866\"},{\"key\":\"return_amount\",\"value\":\"214852806\"},{\"key\":\"tax_amount\",\"value\":\"0\"},{\"key\":\"spread_amount\",\"value\":\"2149\"},{\"key\":\"commission_amount\",\"value\":\"646497\"},{\"key\":\"maker_fee_amount\",\"value\":\"215477\"},{\"key\":\"_contract_address\",\"value\":\"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26\"},{\"key\":\"action\",\"value\":\"transfer\"},{\"key\":\"from\",\"value\":\"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6\"},{\"key\":\"to\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"214852806\"},{\"key\":\"_contract_address\",\"value\":\"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26\"},{\"key\":\"action\",\"value\":\"transfer\"},{\"key\":\"from\",\"value\":\"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6\"},{\"key\":\"to\",\"value\":\"terra1ygcvxp9s054q8u2q4hvl52ke393zvgj0sllahlycm4mj8dm96zjsa45rzk\"},{\"key\":\"amount\",\"value\":\"215477\"}]}]},{\"msg_index\":2,\"events\":[{\"type\":\"coin_received\",\"attributes\":[{\"key\":\"receiver\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"13609525uluna\"}]},{\"type\":\"coin_spent\",\"attributes\":[{\"key\":\"spender\",\"value\":\"terra1e6t37fgjkxrzdx2s95fyq6jdra5s82720vhtmxvx4yhcvnsrey4ssmrya6\"},{\"key\":\"amount\",\"value\":\"13609525uluna\"}]},{\"type\":\"execute\",\"attributes\":[{\"key\":\"_contract_address\",\"value\":\"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26\"},{\"key\":\"_contract_address\",\"value\":\"terra1e6t37fgjkxrzdx2s95fyq6jdra5s82720vhtmxvx4yhcvnsrey4ssmrya6\"}]},{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmwasm.wasm.v1.MsgExecuteContract\"},{\"key\":\"module\",\"value\":\"wasm\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"}]},{\"type\":\"transfer\",\"attributes\":[{\"key\":\"recipient\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"sender\",\"value\":\"terra1e6t37fgjkxrzdx2s95fyq6jdra5s82720vhtmxvx4yhcvnsrey4ssmrya6\"},{\"key\":\"amount\",\"value\":\"13609525uluna\"}]},{\"type\":\"wasm\",\"attributes\":[{\"key\":\"_contract_address\",\"value\":\"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26\"},{\"key\":\"action\",\"value\":\"send\"},{\"key\":\"from\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"to\",\"value\":\"terra1e6t37fgjkxrzdx2s95fyq6jdra5s82720vhtmxvx4yhcvnsrey4ssmrya6\"},{\"key\":\"amount\",\"value\":\"214852805\"},{\"key\":\"_contract_address\",\"value\":\"terra1e6t37fgjkxrzdx2s95fyq6jdra5s82720vhtmxvx4yhcvnsrey4ssmrya6\"},{\"key\":\"action\",\"value\":\"swap\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"receiver\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"offer_asset\",\"value\":\"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26\"},{\"key\":\"ask_asset\",\"value\":\"uluna\"},{\"key\":\"offer_amount\",\"value\":\"214852805\"},{\"key\":\"return_amount\",\"value\":\"13609525\"},{\"key\":\"spread_amount\",\"value\":\"6205\"},{\"key\":\"swap_fee_amount\",\"value\":\"27300\"},{\"key\":\"protocol_fee_amount\",\"value\":\"13650\"},{\"key\":\"burn_fee_amount\",\"value\":\"0\"}]}]},{\"msg_index\":3,\"events\":[{\"type\":\"coin_received\",\"attributes\":[{\"key\":\"receiver\",\"value\":\"terra1d5fzv2y8fpdax4u2nnzrn5uf9ghyu5sxr865uy\"},{\"key\":\"amount\",\"value\":\"6302uluna\"}]},{\"type\":\"coin_spent\",\"attributes\":[{\"key\":\"spender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"6302uluna\"}]},{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmos.bank.v1beta1.MsgSend\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"module\",\"value\":\"bank\"}]},{\"type\":\"transfer\",\"attributes\":[{\"key\":\"recipient\",\"value\":\"terra1d5fzv2y8fpdax4u2nnzrn5uf9ghyu5sxr865uy\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"6302uluna\"}]}]},{\"msg_index\":4,\"events\":[{\"type\":\"coin_received\",\"attributes\":[{\"key\":\"receiver\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"206687395uluna\"}]},{\"type\":\"coin_spent\",\"attributes\":[{\"key\":\"spender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"206687395uluna\"}]},{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmos.bank.v1beta1.MsgSend\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"module\",\"value\":\"bank\"}]},{\"type\":\"transfer\",\"attributes\":[{\"key\":\"recipient\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"sender\",\"value\":\"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf\"},{\"key\":\"amount\",\"value\":\"206687395uluna\"}]}]}]","logs":[{"msg_index":0,"log":"","events":[{"type":"coin_received","attributes":[{"key":"receiver","value":"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr"},{"key":"amount","value":"13603122uluna"}]},{"type":"coin_spent","attributes":[{"key":"spender","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"},{"key":"amount","value":"13603122uluna"}]},{"type":"execute","attributes":[{"key":"_contract_address","value":"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgExecuteContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"}]},{"type":"transfer","attributes":[{"key":"recipient","value":"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr"},{"key":"sender","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"},{"key":"amount","value":"13603122uluna"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr"},{"key":"action","value":"swap"},{"key":"sender","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"},{"key":"receiver","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"},{"key":"offer_asset","value":"uluna"},{"key":"ask_asset","value":"ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4"},{"key":"offer_amount","value":"13603122"},{"key":"return_amount","value":"17768867"},{"key":"tax_amount","value":"0"},{"key":"spread_amount","value":"111"},{"key":"commission_amount","value":"53466"},{"key":"maker_fee_amount","value":"17820"}]}]},{"msg_index":1,"log":"","events":[{"type":"coin_received","attributes":[{"key":"receiver","value":"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6"},{"key":"amount","value":"17768866ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4"}]},{"type":"coin_spent","attributes":[{"key":"spender","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"},{"key":"amount","value":"17768866ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4"}]},{"type":"execute","attributes":[{"key":"_contract_address","value":"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6"},{"key":"_contract_address","value":"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26"},{"key":"_contract_address","value":"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26"}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgExecuteContract"},{"key":"module","value":"wasm"},{"key":"sender","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"}]},{"type":"transfer","attributes":[{"key":"recipient","value":"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6"},{"key":"sender","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"},{"key":"amount","value":"17768866ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4"}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6"},{"key":"action","value":"swap"},{"key":"sender","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"},{"key":"receiver","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"},{"key":"offer_asset","value":"ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4"},{"key":"ask_asset","value":"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26"},{"key":"offer_amount","value":"17768866"},{"key":"return_amount","value":"214852806"},{"key":"tax_amount","value":"0"},{"key":"spread_amount","value":"2149"},{"key":"commission_amount","value":"646497"},{"key":"maker_fee_amount","value":"215477"},{"key":"_contract_address","value":"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26"},{"key":"action","value":"transfer"},{"key":"from","value":"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6"},{"key":"to","value":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf"},{"key":"amount","value":"214852806"},{"key":"_contract_address","value":"terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26"},{"key":"action","value":"transfer"},{"key":"from","value":"terra1w579ysjvpx7xxhckxewk8sykxz70gm48wpcuruenl29rhe6p6raslhj0m6"},{"key":"to","value":"terra1ygcvxp9s054q8u2q4hvl52ke393zvgj0sllahlycm4mj8dm96zjsa45rzk"},{"key":"amount","value":"215477"}]}]}],"info":"","gas_wanted":"1800000","gas_used":"1040027","tx":{"@type":"/cosmos.tx.v1beta1.Tx","body":{"messages":[{"@type":"/cosmwasm.wasm.v1.MsgExecuteContract","sender":"terra1cy7sn7dvruu49wchuwlqdl38vpgtf5at8hf2nf","contract":"terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr","msg":{"swap":{"max_spread":"0.02","offer_asset":{"amount":"13603122","info":{"native_token":{"denom":"uluna"}}},"belief_price":"0.7655593778466223"}},"funds":[{"denom":"uluna","amount":"13603122"}]}],"memo":"","timeout_height":"0","extension_options":[],"non_critical_extension_options":[]},"auth_info":{"signer_infos":[{"public_key":{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A8OIhK6KVPeDnE3xYiqFP90vb8PzKVDCsJJCtDuGKXNq"},"mode_info":{"single":{"mode":"SIGN_MODE_DIRECT"}},"sequence":"2893"}],"fee":{"amount":[{"denom":"uluna","amount":"45"}],"gas_limit":"1800000","payer":"","granter":""}},"signatures":["8L4ePhdODQ4ro97FeefrkbVHGfNofViKDAga1GJ6NIdQudf1cWEzdSFyIZqM1nVA4CGCQRMEOR3arHk2OLT7WA=="]},"timestamp":"2023-04-07T00:27:04Z","events":[{"type":"coin_spent","attributes":[{"key":"c3BlbmRlcg==","value":"dGVycmExY3k3c243ZHZydXU0OXdjaHV3bHFkbDM4dnBndGY1YXQ4aGYybmY=","index":true},{"key":"YW1vdW50","value":"NDV1bHVuYQ==","index":true}]},{"type":"coin_received","attributes":[{"key":"cmVjZWl2ZXI=","value":"dGVycmExN3hwZnZha20yYW1nOTYyeWxzNmY4NHoza2VsbDhjNWxrYWVxZmE=","index":true},{"key":"YW1vdW50","value":"NDV1bHVuYQ==","index":true}]},{"type":"transfer","attributes":[{"key":"cmVjaXBpZW50","value":"dGVycmExN3hwZnZha20yYW1nOTYyeWxzNmY4NHoza2VsbDhjNWxrYWVxZmE=","index":true},{"key":"c2VuZGVy","value":"dGVycmExY3k3c243ZHZydXU0OXdjaHV3bHFkbDM4dnBndGY1YXQ4aGYybmY=","index":true},{"key":"YW1vdW50","value":"NDV1bHVuYQ==","index":true}]},{"type":"message","attributes":[{"key":"c2VuZGVy","value":"dGVycmExY3k3c243ZHZydXU0OXdjaHV3bHFkbDM4dnBndGY1YXQ4aGYybmY=","index":true}]},{"type":"tx","attributes":[{"key":"ZmVl","value":"NDV1bHVuYQ==","index":true},{"key":"ZmVlX3BheWVy","value":"dGVycmExY3k3c243ZHZydXU0OXdjaHV3bHFkbDM4dnBndGY1YXQ4aGYybmY=","index":true}]},{"type":"wasm","attributes":[{"key":"X2NvbnRyYWN0X2FkZHJlc3M=","value":"dGVycmExZmQ2OGFoMDJncjJ5OHplN3RtOXRlN203MHpsbWM3dmp5eWhzNnhsaHNkbXFxY2p1ZDRkcWw0d3B4cg==","index":true}]},{"type":"execute","attributes":[{"key":"X2NvbnRyYWN0X2FkZHJlc3M=","value":"dGVycmExdzU3OXlzanZweDd4eGhja3hld2s4c3lreHo3MGdtNDh3cGN1cnVlbmwyOXJoZTZwNnJhc2xoajBtNg==","index":true}]},{"type":"message","attributes":[{"key":"bW9kdWxl","value":"YmFuaw==","index":true}]}]}}"#;

#[test]
fn tx_resp() {
    let data: Value = serde_json::from_str(TEST_TX.trim()).unwrap();

    let tx_response = data.as_object().unwrap().get("tx_response").unwrap();

    let height: u64 = tx_response
        .get("height")
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let txhash: String = String::from(tx_response.get("txhash").unwrap().as_str().unwrap());
    let data: String = String::from(tx_response.get("data").unwrap().as_str().unwrap());
    let raw_log: String = String::from(tx_response.get("raw_log").unwrap().as_str().unwrap());

    let logs: Vec<TxResultBlockMsg> = tx_response
        .get("logs")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(TxResultBlockMsg::from)
        .collect::<Vec<TxResultBlockMsg>>();

    let info: String = String::from(tx_response.get("info").unwrap().as_str().unwrap());

    let gas_wanted: u64 = tx_response
        .get("gas_wanted")
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let gas_used: u64 = tx_response
        .get("gas_used")
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let events: Vec<Event> = tx_response
        .get("events")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|data| {
            let attributes = data
                .get("attributes")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|attr| EventAttribute {
                    key: String::from(attr.get("key").unwrap().as_str().unwrap()),
                    value: String::from(attr.get("value").unwrap().as_str().unwrap()),
                    index: attr.get("index").unwrap().as_bool().unwrap(),
                })
                .collect::<Vec<EventAttribute>>();

            Event {
                r#type: String::from(data.get("type").unwrap().as_str().unwrap()),
                attributes,
            }
        })
        .collect::<Vec<Event>>();

    let stamp = tx_response
        .get("timestamp")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let timestamp = parse_timestamp(stamp).unwrap();

    let tx_res = CosmTxResponse {
        height,
        txhash,
        codespace: String::from(""),
        code: 0,
        data,
        raw_log,
        logs,
        info,
        gas_wanted,
        gas_used,
        timestamp,
        events,
    };

    let res = tx_res.get_attribute_from_logs("coin_received", "receiver");
    asserting!("get_attribute_from_logs returns correct amount")
        .that(&res.len())
        .is_equal_to(2);

    let res = tx_res.get_events("wasm");
    asserting!("get_events returns the correct amount")
        .that(&res.len())
        .is_equal_to(2);

    let attrs = res[0].get_attributes("receiver");
    asserting!("get_attributes returns the correct amount")
        .that(&attrs.len())
        .is_equal_to(1);

    let value = res[0].get_first_attribute_value("_contract_address");
    asserting!("get_first_value response is present")
        .that(&value)
        .is_some();

    let res = tx_res.events();
    asserting!("IndexResponse events returns the correct amount")
        .that(&res.len())
        .is_equal_to(8);

    asserting!("IndexResponse data is present")
        .that(&tx_res.data())
        .is_some();

    let res = tx_res.event_attr_value("coin_spent", "c3BlbmRlcg==");
    asserting!("IndexResponse events returns value")
        .that(&res)
        .is_ok();
}

#[test]
fn test_timestamp() {
    let timestamp = parse_timestamp(String::from("2023-04-07T00:27:04")).unwrap();

    let ts_time = timestamp.time();
    asserting!("timestamp time is equal to dataset timestamp")
        .that(&ts_time.to_string())
        .is_equal_to(&String::from("00:27:04"));

    let ts_date = timestamp.date_naive();
    asserting!("timestamp date is equal to dataset timestamp")
        .that(&ts_date.to_string())
        .is_equal_to(&String::from("2023-04-07"));
}

#[test]
fn test_breaking_timestamp() {
    let timestamp = parse_timestamp(String::from("2023-04-09T18:30:45-07:00"));
    asserting!("timestamp is invalid").that(&timestamp).is_err();
}

#[test]
fn test_breaking_event_attr_value() {
    let tx_res = CosmTxResponse::default();

    let res = tx_res.event_attr_value("i_dont_exists", "something");
    asserting!("it errors because attribute does not exists")
        .that(&res)
        .is_err();
}

#[test]
fn test_breaking_data_is_none() {
    let tx_res = CosmTxResponse::default();

    asserting!("that data is None")
        .that(&tx_res.data())
        .is_none();
}
