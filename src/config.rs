#[derive(Serialize,Deserialize,Debug)]
pub struct Config {
    pub polling_rate: u64, //seconds
    pub charge_threshold: u8, //%
    pub warning_period: u64 //seconds
}
