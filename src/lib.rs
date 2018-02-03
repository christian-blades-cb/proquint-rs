use std::error;
use std::fmt::{Display, Formatter};
use std::fmt;

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

pub trait Quintable where Self: Sized{
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

macro_rules! cons_u32 {
    ($i:ident, $out:ident) => {
        let j: u32 = ($i & MASK_FIRST4_U32) >> 28;
        $i <<= 4;
        $out.push(UINT2CONSONANT[j as usize]);
    }
}

macro_rules! vowel_u32 {
    ($i:ident, $out:ident) => {
        let j: u32 = ($i & MASK_FIRST2_U32) >> 30;
        $i <<= 2;
        $out.push(UINT2VOWEL[j as usize]);
    }
}

const UINT2CONSONANT: &'static [char] =
    &['b', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n',
      'p', 'r', 's', 't', 'v', 'z'];
const UINT2VOWEL: &'static [char] = &['a', 'i', 'o', 'u'];
const MASK_FIRST4_U32: u32 = 0xF0000000;
const MASK_FIRST2_U32: u32 = 0xC0000000;
const MASK_FIRST4_U16: u16 = 0xF000;
const MASK_FIRST2_U16: u16 = 0xC000;

const SEPARATOR: char = '-';

impl Quintable for u16 {
    fn to_quint(&self) -> String {
        let mut out = String::new();
        let mut i = self.to_owned();

        cons_u16!(i, out);
        vowel_u16!(i, out);
        cons_u16!(i, out);
        vowel_u16!(i, out);
        cons_u16!(i, out);

        out
    }

    fn from_quint(quint: &str) -> Result<u16, QuintError> {
        let mut res: u16 = 0;
        
        for c in quint.chars() {
            match c {
                // consonants
                'b' => decons!(res, 0),                
                'd' => decons!(res, 1),
                'f' => decons!(res, 2),
                'g' => decons!(res, 3),
                'h' => decons!(res, 4),
                'j' => decons!(res, 5),
                'k' => decons!(res, 6),
                'l' => decons!(res, 7),
                'm' => decons!(res, 8),
                'n' => decons!(res, 9),
                'p' => decons!(res, 10),
                'r' => decons!(res, 11),
                's' => decons!(res, 12),
                't' => decons!(res, 13),
                'v' => decons!(res, 14),
                'z' => decons!(res, 15),
                
                // vowels
                'a' => devowel!(res, 0),
                'i' => devowel!(res, 1),
                'o' => devowel!(res, 2),
                'u' => devowel!(res, 3),

                // separators
                _ => (),
            }
        }

        Ok(res)
    }
}

impl Quintable for u32 {
    fn to_quint(&self) -> String {
        let mut out = String::new();
        let mut i = self.to_owned();

        cons_u32!(i, out);
        vowel_u32!(i, out);
        cons_u32!(i, out);
        vowel_u32!(i, out);
        cons_u32!(i, out);

        out.push(SEPARATOR);

        cons_u32!(i, out);
        vowel_u32!(i, out);
        cons_u32!(i, out);
        vowel_u32!(i, out);
        cons_u32!(i, out);        

        out        
    }

    fn from_quint(quint: &str) -> Result<u32, QuintError> {
        let mut res: u32 = 0;
        for c in quint.chars() {
            match c {
                // consonants
                'b' => decons!(res, 0),                
                'd' => decons!(res, 1),
                'f' => decons!(res, 2),
                'g' => decons!(res, 3),
                'h' => decons!(res, 4),
                'j' => decons!(res, 5),
                'k' => decons!(res, 6),
                'l' => decons!(res, 7),
                'm' => decons!(res, 8),
                'n' => decons!(res, 9),
                'p' => decons!(res, 10),
                'r' => decons!(res, 11),
                's' => decons!(res, 12),
                't' => decons!(res, 13),
                'v' => decons!(res, 14),
                'z' => decons!(res, 15),
                
                // vowels
                'a' => devowel!(res, 0),
                'i' => devowel!(res, 1),
                'o' => devowel!(res, 2),
                'u' => devowel!(res, 3),

                // separators
                _ => (),
            }
        }

        Ok(res)
    }
}

impl Quintable for std::net::Ipv4Addr {
    fn to_quint(&self) -> String {
        let octets = self.octets();
        let as_int: u32 =
            octets[3] as u32 |
            (octets[2] as u32) << 8 |
            (octets[1] as u32) << 16 |
            (octets[0] as u32) << 24;
        
        as_int.to_quint()
    }

    fn from_quint(quint: &str) -> Result<std::net::Ipv4Addr, QuintError> {
        let as_int: u32 = u32::from_quint(quint)?;

        let first = as_int >> 24;
        let second = (as_int & 0x00FF0000) >> 16;
        let third = (as_int & 0x0000FF00) >> 8;
        let fourth = as_int & 0x000000FF;

        Ok(std::net::Ipv4Addr::new(first as u8,
                                   second as u8,
                                   third as u8,
                                   fourth as u8))
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use Quintable;
    
    #[test]
    fn ipv4_quints() {
        assert_eq!(Ipv4Addr::new(127, 0, 0, 1).to_quint(), "lusab-babad");
        let ipv4: Ipv4Addr = Ipv4Addr::from_quint("lusab-babad").unwrap();
        assert_eq!(ipv4, Ipv4Addr::new(127, 0, 0, 1));
    }
}
