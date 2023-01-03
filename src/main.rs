use cryptography::sha::sha3::Sha3;

fn main() {
    let mut s = Sha3::new(vec![]);

    println!("{}", s.keccak(256));
}
