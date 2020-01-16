type Error = unsigned_varint::decode::Error;

pub trait Deserialize: Sized {
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), Error>;
}

macro_rules! impl_deser {
    ($($t:ident),+ $(,)?) => {
        $(
        impl Deserialize for $t {
            fn decode(buf: &[u8]) -> Result<(Self, &[u8]), Error> {
                unsigned_varint::decode::$t(buf)
            }
        }
        )+
    }
}

impl_deser!(u8, u16, u32, u64, u128, usize);
