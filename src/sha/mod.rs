pub mod sha1;
pub mod sha256;
pub mod sha3;
pub mod sha512;

use self::{sha1::Sha1, sha256::Sha256, sha512::Sha512};

pub fn print_digest(sha: usize, msg: &str) -> String {
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
        // x if x > 3_000_000 => Sha3::new(msg.as_bytes(), x - 3_000_000 - x % 1000, x % 1000)
        //     .result()
        //     .iter()
        //     .fold("".to_string(), |acc, x| format!("{acc}{:x}", *x)),
        _ => panic!("Not yet implemented!"),
    }
}
