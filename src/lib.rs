//! Library for converting to and from proquints.
//!
//! # Proquint
//!
//! A proquint is a pronouncable representation of an identifier, such
//! as an IP address, document number, user id, etc. The purpose is to
//! provide a more convenient way for humans to
//! interact/remember/communicate with unique identifiers.
//!
//! Original proposal found here: https://arxiv.org/html/0901.4016
//!
//! # Example
//! ```
//! use proquint::Quintable;
//! use std::net::Ipv4Addr;
//!
//! let home = Ipv4Addr::new(127, 0, 0, 1);
//! assert_eq!(home.to_quint(), "lusab-babad");
//! ```

use std::error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{ShlAssign, AddAssign};

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[derive(Debug,PartialEq)]
pub enum QuintError {
    InputTooSmall,
    InputTooLarge,
    InputInvalid,
}

impl Display for QuintError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let out = match *self {
            QuintError::InputTooLarge => "proquint was too large",
            QuintError::InputTooSmall => "expected larger proquint",
            QuintError::InputInvalid => "input was not a valid proquint",
        };
        write!(f, "{}", out)
    }
}

impl error::Error for QuintError {
    fn description(&self) -> &str {
        match *self {
            QuintError::InputTooLarge => "proquint was too large",
            QuintError::InputTooSmall => "expected larger proquint",
            QuintError::InputInvalid => "input was not a valid proquint",
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
    /// Converts this type into a proquint String
    ///
    /// # Example
    /// ```
    /// use proquint::Quintable;
    ///
    /// let foo: u32 = 12;
    /// assert_eq!(foo.to_quint(), "babab-babas");
    /// ```
    fn to_quint(&self) -> String;

    /// Converts a proquint string to this type
    ///
    /// # Example
    /// ```
    /// use proquint::Quintable;
    ///
    /// assert_eq!(u32::from_quint("rotab-vinat").unwrap(), 3141592653u32);
    /// ```
    fn from_quint(&str) -> Result<Self, QuintError>;
}

macro_rules! decons {
    ($res:ident, $bitcounter:ident, $x:expr) => {{
        $bitcounter += 4;
        $res <<= 4;
        $res += $x;
    }}
}

macro_rules! devowel {
    ($res:ident, $bitcounter:ident, $x:expr) => {{
        $bitcounter += 2;
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

/// Generic function for converting a proquint string to the given type.
///
/// Returns the decoded type as well as the number of bits decoded. A full proquint is 16 bits, so a valid proquint will be a multiple of this size.
pub fn from_quint<T>(quint: &str) -> (T, usize)
    where T: Sized + Default + ShlAssign<isize> + AddAssign<T> + From<u8>
{
    let mut bitcounter = 0usize;
    let mut res: T = T::default();
    for c in quint.chars() {
        match c {
            // consonants
            'b' => decons!(res, bitcounter, T::from(0u8)),
            'd' => decons!(res, bitcounter, T::from(1u8)),
            'f' => decons!(res, bitcounter, T::from(2u8)),
            'g' => decons!(res, bitcounter, T::from(3u8)),
            'h' => decons!(res, bitcounter, T::from(4u8)),
            'j' => decons!(res, bitcounter, T::from(5u8)),
            'k' => decons!(res, bitcounter, T::from(6u8)),
            'l' => decons!(res, bitcounter, T::from(7u8)),
            'm' => decons!(res, bitcounter, T::from(8u8)),
            'n' => decons!(res, bitcounter, T::from(9u8)),
            'p' => decons!(res, bitcounter, T::from(10u8)),
            'r' => decons!(res, bitcounter, T::from(11u8)),
            's' => decons!(res, bitcounter, T::from(12u8)),
            't' => decons!(res, bitcounter, T::from(13u8)),
            'v' => decons!(res, bitcounter, T::from(14u8)),
            'z' => decons!(res, bitcounter, T::from(15u8)),

            // vowels
            'a' => devowel!(res, bitcounter, T::from(0u8)),
            'i' => devowel!(res, bitcounter, T::from(1u8)),
            'o' => devowel!(res, bitcounter, T::from(2u8)),
            'u' => devowel!(res, bitcounter, T::from(3u8)),

            // separators
            _ => (),
        }
    }

    (res, bitcounter)
}

pub fn unquint_exactly<T>(quint: &str, bits: usize) -> Result<(T, usize), QuintError>
    where T: Sized + Default + ShlAssign<isize> + AddAssign<T> + From<u8>
{
    let mut bitcounter = 0usize;
    let mut res: T = T::default();
    let mut final_idx = 0usize;
    for (i, c) in quint.chars().enumerate() {
        match c {
            // consonants
            'b' => decons!(res, bitcounter, T::from(0u8)),
            'd' => decons!(res, bitcounter, T::from(1u8)),
            'f' => decons!(res, bitcounter, T::from(2u8)),
            'g' => decons!(res, bitcounter, T::from(3u8)),
            'h' => decons!(res, bitcounter, T::from(4u8)),
            'j' => decons!(res, bitcounter, T::from(5u8)),
            'k' => decons!(res, bitcounter, T::from(6u8)),
            'l' => decons!(res, bitcounter, T::from(7u8)),
            'm' => decons!(res, bitcounter, T::from(8u8)),
            'n' => decons!(res, bitcounter, T::from(9u8)),
            'p' => decons!(res, bitcounter, T::from(10u8)),
            'r' => decons!(res, bitcounter, T::from(11u8)),
            's' => decons!(res, bitcounter, T::from(12u8)),
            't' => decons!(res, bitcounter, T::from(13u8)),
            'v' => decons!(res, bitcounter, T::from(14u8)),
            'z' => decons!(res, bitcounter, T::from(15u8)),

            // vowels
            'a' => devowel!(res, bitcounter, T::from(0u8)),
            'i' => devowel!(res, bitcounter, T::from(1u8)),
            'o' => devowel!(res, bitcounter, T::from(2u8)),
            'u' => devowel!(res, bitcounter, T::from(3u8)),

            // separators
            _ => (),
        }
        if bitcounter >= bits {
            final_idx = i;
            break;
        }
    }

    if bitcounter == bits {
        return Ok((res, final_idx));
    }

    if bitcounter >  bits {
        Err(QuintError::InputInvalid)
    } else {
        Err(QuintError::InputTooSmall)
    }
}

macro_rules! impl_from_quint {
    ($expected_bits:expr) => {
        fn from_quint(quint: &str) -> Result<Self, QuintError> {
            let (res, bits) = from_quint(quint);
            if bits == $expected_bits {
                return Ok(res);
            }
            if bits < $expected_bits {
                return Err(QuintError::InputTooSmall);
            } else {
                return Err(QuintError::InputTooLarge);
            }
        }
    }
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

    impl_from_quint!(16);
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

    impl_from_quint!(32);
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

    impl_from_quint!(64);
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

impl Quintable for std::net::Ipv6Addr {
    fn to_quint(&self) -> String {
        let segments: [u16; 8] = self.segments();
        let out: Vec<String> = segments.iter().map(|s| s.to_quint()).collect();
        out.join("-")
    }

    fn from_quint(quint: &str) -> Result<Self, QuintError> {
        let q = &quint;
        let (first, last_i) = unquint_exactly(q, 16)?;
        let q = &q[last_i+1..];
        let (second, last_i) = unquint_exactly(q, 16)?;
        let q = &q[last_i+1..];
        let (third, last_i) = unquint_exactly(q, 16)?;
        let q = &q[last_i+1..];
        let (fourth, last_i) = unquint_exactly(q, 16)?;
        let q = &q[last_i+1..];
        let (fifth, last_i) = unquint_exactly(q, 16)?;
        let q = &q[last_i+1..];
        let (sixth, last_i) = unquint_exactly(q, 16)?;
        let q = &q[last_i+1..];
        let (seventh, last_i) = unquint_exactly(q, 16)?;
        let q = &q[last_i+1..];
        let (eighth, _) = unquint_exactly(q, 16)?;

        Ok(std::net::Ipv6Addr::new(first, second, third, fourth, fifth, sixth, seventh, eighth))
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};
    use Quintable;
    use QuintError;

    #[test]
    fn quint_too_small() {
        assert_eq!(u16::from_quint("lub").err(), Some(QuintError::InputTooSmall));
        assert_eq!(u32::from_quint("lubab").err(), Some(QuintError::InputTooSmall));
        assert_eq!(u64::from_quint("lubab-gutuz").err(), Some(QuintError::InputTooSmall));
    }

    #[test]
    fn quint_too_large() {
        assert_eq!(u16::from_quint("lubab-gutuz").err(), Some(QuintError::InputTooLarge));
        assert_eq!(u32::from_quint("lubab-gutuz-kobim").err(), Some(QuintError::InputTooLarge));
        assert_eq!(u64::from_quint("lubab-gutuz-kobim-fival-bison").err(), Some(QuintError::InputTooLarge));
    }

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

    fn back_and_forth<T>(xs: T) -> bool
        where T: Quintable + PartialEq
    {
        let quint = xs.to_quint();
        let y = match T::from_quint(&quint) {
            Ok(res) => res,
            Err(e) => {
                println!("error! {:?}", e);
                return false;
            },
        };
        xs == y
    }

    quickcheck! {
        fn u16(xs: u16) -> bool {
            back_and_forth(xs)
        }
    }

    quickcheck! {
        fn u32(xs: u32) -> bool {
            back_and_forth(xs)
        }
    }

    quickcheck! {
        fn u64(xs: u64) -> bool {
            back_and_forth(xs)
        }
    }

    quickcheck! {
        fn ipv4(xs: Ipv4Addr) -> bool {
            back_and_forth(xs)
        }
    }

    quickcheck! {
        fn ipv6(xs: Ipv6Addr) -> bool {
            back_and_forth(xs)
        }
    }

}
