use cryptography::sha::sha512::Sha512;

fn main() {
    println!(
        "{}",
        Sha512::new("hello".as_bytes())
            .result()
            .iter()
            .fold("".to_string(), |acc, x| format!("{acc}{:x}", *x))
    )
}
