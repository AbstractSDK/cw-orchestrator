/// Allows cw-orch to query tx by events because of the API change described in :
/// https://github.com/cosmos/cosmos-sdk/blob/b48fd66678a98b915888cc84976399ac17164370/CHANGELOG.md?plain=1#L595
/// TODO : Remove when cosmos-rs is updated (current version supported v0.46)
/// https://github.com/cosmos/cosmos-rust/blob/main/cosmos-sdk-proto/src/prost/cosmos-sdk/COSMOS_SDK_COMMIT
pub mod v0_50;
