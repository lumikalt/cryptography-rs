use cryptography::sha::print_digest;

fn main() {
    println!("Sha256 for \"hello\":\n{}", print_digest(256, "hello"))
}
