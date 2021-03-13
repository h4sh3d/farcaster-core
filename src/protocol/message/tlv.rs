#[derive(Clone, Debug, Api)]
#[api(encoding = "lightning")]
pub enum Message {
    #[api(type = 0x0001)]
    Hello(String),

    /// Some attribute
    #[api(type = 0x0003)]
    Empty(),

    #[api(type = 0x0005)]
    NoArgs,

    #[api(type = 0x0103)]
    AddKeys(u8),
}
