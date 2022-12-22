use crate::sha::sha512::Sha512;

use self::{sha1::Sha1, sha256::Sha256};

pub mod sha1;
pub mod sha256;
pub mod sha512;

pub fn print_digest(sha: u16, msg: &str) -> String {
    match sha {
        1 => Sha1::new(msg.as_bytes())
            .result()
            .iter()
            .fold("".to_string(), |acc, x| format!("{acc}{:x}", *x)),
        256 => Sha256::new(msg.as_bytes())
            .result()
            .iter()
            .fold("".to_string(), |acc, x| format!("{acc}{:x}", *x)),
        512 => Sha512::new(msg.as_bytes())
            .result()
            .iter()
            .fold("".to_string(), |acc, x| format!("{acc}{:x}", *x)),
        _ => panic!("Not yet implemented!"),
    }
}
