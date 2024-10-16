#[derive(Serialize,Deserialize,Debug)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub status: String,
}
