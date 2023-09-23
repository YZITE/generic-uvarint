use generic_array::typenum;
use std::convert::TryInto;

pub trait Serialize: Copy + Sized {
    type MaxEncLen: generic_array::ArrayLength;

    fn encode(self, buf: &mut generic_array::GenericArray<u8, Self::MaxEncLen>) -> &[u8];
}

macro_rules! impl_ser {
    ($($t:ident => $n:ident),+ $(,)?) => {
        $(
        impl Serialize for $t {
            type MaxEncLen = typenum::$n;

            #[inline]
            fn encode(self, buf: &mut generic_array::GenericArray<u8, Self::MaxEncLen>) -> &[u8] {
                unsigned_varint::encode::$t(self, buf.as_mut_slice().try_into().unwrap())
            }
        }
        )+
    }
}

impl_ser!(
    u8 => U2,
    u16 => U3,
    u32 => U5,
    u64 => U10,
    u128 => U19,
);

#[cfg(target_pointer_width = "64")]
impl_ser!(usize => U10);

#[cfg(target_pointer_width = "32")]
impl_ser!(usize => U5);

pub fn encode_uint<'a, T: Serialize>(x: T, buf: &'a mut [u8]) -> &'a [u8]
where
    <T as Serialize>::MaxEncLen: 'a,
{
    Serialize::encode(x, buf.try_into().expect("got invalid slice length"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn _u32() {
        let mut buf = [0u8; 5];
        let _ = super::encode_uint(100u32, &mut buf);
    }
}
