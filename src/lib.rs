use std::error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{ShlAssign, AddAssign};

#[derive(Debug)]
pub enum QuintError {
    InputTooSmall,
    InputTooLarge,
}

impl Display for QuintError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let out = match *self {
            QuintError::InputTooLarge => "proquint was too large",
            QuintError::InputTooSmall => "expected larger proquint",
        };
        write!(f, "{}", out)
    }
}

impl error::Error for QuintError {
    fn description(&self) -> &str {
        match *self {
            QuintError::InputTooLarge => "proquint was too large",
            QuintError::InputTooSmall => "expected larger proquint",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// Trait for values that can be converted to proquints
///
/// More about proquints here at the original proposal: https://arxiv.org/html/0901.4016
pub trait Quintable
    where Self: Sized
{
    fn to_quint(&self) -> String;
    fn from_quint(&str) -> Result<Self, QuintError>;
}

macro_rules! decons {
    ($res:ident, $x:expr) => {{
        $res <<= 4;
        $res += $x;
    }}
}

macro_rules! devowel {
    ($res:ident, $x:expr) => {{
        $res <<= 2;
        $res += $x;
    }}
}

macro_rules! cons_u16 {
    ($i:ident, $out:ident) => {
        let j: u16 = ($i & MASK_FIRST4_U16) >> 12;
        $i <<= 4;
        $out.push(UINT2CONSONANT[j as usize]);
    }
}

macro_rules! vowel_u16 {
    ($i:ident, $out:ident) => {
        let j: u16 = ($i & MASK_FIRST2_U16) >> 14;
        $i <<= 2;
        $out.push(UINT2VOWEL[j as usize]);
    }
}

const UINT2CONSONANT: &'static [char] = &['b', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p',
                                          'r', 's', 't', 'v', 'z'];
const UINT2VOWEL: &'static [char] = &['a', 'i', 'o', 'u'];

const MASK_FIRST4_U16: u16 = 0xF000;
const MASK_FIRST2_U16: u16 = 0xC000;

const SEPARATOR: char = '-';

pub fn from_quint<T>(quint: &str) -> Result<T, QuintError>
    where T: Sized + Default + ShlAssign<isize> + AddAssign<T> + From<u8>
{
    let mut res: T = T::default();
    for c in quint.chars() {
        match c {
            // consonants
            'b' => decons!(res, T::from(0u8)),
            'd' => decons!(res, T::from(1u8)),
            'f' => decons!(res, T::from(2u8)),
            'g' => decons!(res, T::from(3u8)),
            'h' => decons!(res, T::from(4u8)),
            'j' => decons!(res, T::from(5u8)),
            'k' => decons!(res, T::from(6u8)),
            'l' => decons!(res, T::from(7u8)),
            'm' => decons!(res, T::from(8u8)),
            'n' => decons!(res, T::from(9u8)),
            'p' => decons!(res, T::from(10u8)),
            'r' => decons!(res, T::from(11u8)),
            's' => decons!(res, T::from(12u8)),
            't' => decons!(res, T::from(13u8)),
            'v' => decons!(res, T::from(14u8)),
            'z' => decons!(res, T::from(15u8)),

            // vowels
            'a' => devowel!(res, T::from(0u8)),
            'i' => devowel!(res, T::from(1u8)),
            'o' => devowel!(res, T::from(2u8)),
            'u' => devowel!(res, T::from(3u8)),

            // separators
            _ => (),
        }
    }

    Ok(res)
}

impl Quintable for u16 {
    fn to_quint(&self) -> String {
        let mut out = String::with_capacity(5);
        let mut i = self.to_owned();

        cons_u16!(i, out);
        vowel_u16!(i, out);
        cons_u16!(i, out);
        vowel_u16!(i, out);
        // final consonant
        let j: u16 = (i & MASK_FIRST4_U16) >> 12;
        out.push(UINT2CONSONANT[j as usize]);

        out
    }

    fn from_quint(quint: &str) -> Result<Self, QuintError> {
        from_quint(quint)
    }
}

impl Quintable for u32 {
    fn to_quint(&self) -> String {
        let mut out = String::with_capacity(11);
        let first = ((self & 0xFFFF0000) >> 16) as u16;
        let second = (self & 0x0000FFFF) as u16;

        out.push_str(&first.to_quint());
        out.push(SEPARATOR);
        out.push_str(&second.to_quint());

        out
    }

    fn from_quint(quint: &str) -> Result<Self, QuintError> {
        from_quint(quint)
    }
}

impl Quintable for u64 {
    fn to_quint(&self) -> String {
        let mut out = String::with_capacity(23);
        let first = ((self & 0xFFFF000000000000) >> 48) as u16;
        let second = ((self & 0x0000FFFF00000000) >> 32) as u16;
        let third = ((self & 0x00000000FFFF0000) >> 16) as u16;
        let fourth = (self & 0x000000000000FFFF) as u16;

        out.push_str(&first.to_quint());
        out.push(SEPARATOR);
        out.push_str(&second.to_quint());
        out.push(SEPARATOR);
        out.push_str(&third.to_quint());
        out.push(SEPARATOR);
        out.push_str(&fourth.to_quint());

        out
    }

    fn from_quint(quint: &str) -> Result<Self, QuintError> {
        from_quint(quint)
    }
}

impl Quintable for std::net::Ipv4Addr {
    fn to_quint(&self) -> String {
        let octets = self.octets();
        let as_int: u32 = octets[3] as u32 | (octets[2] as u32) << 8 | (octets[1] as u32) << 16 |
                          (octets[0] as u32) << 24;

        as_int.to_quint()
    }

    fn from_quint(quint: &str) -> Result<std::net::Ipv4Addr, QuintError> {
        let as_int: u32 = u32::from_quint(quint)?;

        let first = as_int >> 24;
        let second = (as_int & 0x00FF0000) >> 16;
        let third = (as_int & 0x0000FF00) >> 8;
        let fourth = as_int & 0x000000FF;

        Ok(std::net::Ipv4Addr::new(first as u8, second as u8, third as u8, fourth as u8))
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use Quintable;

    fn ipv4_test(ipv4: [u8; 4], quint: &str) {
        assert_eq!(Ipv4Addr::from(ipv4).to_quint(), quint);

        let actual = Ipv4Addr::from_quint(quint).unwrap();
        assert_eq!(actual, Ipv4Addr::from(ipv4));
    }

    #[test]
    fn ipv4_quints() {
        // 127.0.0.1       lusab-babad
        ipv4_test([127, 0, 0, 1], "lusab-babad");
        // 63.84.220.193   gutih-tugad
        ipv4_test([63, 84, 220, 193], "gutih-tugad");
        // 63.118.7.35     gutuk-bisog
        ipv4_test([63, 118, 7, 35], "gutuk-bisog");
        // 140.98.193.141  mudof-sakat
        ipv4_test([140, 98, 193, 141], "mudof-sakat");
        // 64.255.6.200    haguz-biram
        ipv4_test([64, 255, 6, 200], "haguz-biram");
        // 128.30.52.45    mabiv-gibot
        ipv4_test([128, 30, 52, 45], "mabiv-gibot");
        // 147.67.119.2    natag-lisaf
        ipv4_test([147, 67, 119, 2], "natag-lisaf");
        // 212.58.253.68   tibup-zujah
        ipv4_test([212, 58, 253, 68], "tibup-zujah");
        // 216.35.68.215   tobog-higil
        ipv4_test([216, 35, 68, 215], "tobog-higil");
        // 216.68.232.21   todah-vobij
        ipv4_test([216, 68, 232, 21], "todah-vobij");
        // 198.81.129.136  sinid-makam
        ipv4_test([198, 81, 129, 136], "sinid-makam");
        // 12.110.110.204  budov-kuras
        ipv4_test([12, 110, 110, 204], "budov-kuras");
    }
}
