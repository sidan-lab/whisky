#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSlotConfig {
    pub slot_length: u32,
    pub zero_slot: u64,
    pub zero_time: u64,
}
