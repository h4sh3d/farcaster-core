
#[derive(Clone, Debug, Api)] //
#[api(encoding = "lightning")]
pub enum Message {
    #[api(type = 0x0103)]
    AddKeys(Vec<u8>),
}
