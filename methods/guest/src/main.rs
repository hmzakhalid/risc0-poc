use risc0_zkvm::guest::env;
use fhe::bfv::BfvParametersBuilder;

fn main() {
    // read the input
    let degree: u64 = env::read();
    let plaintext_modulus: u64 = env::read();
    let moduli: Vec<u64> = env::read();

    println!("Degree: {}, Plaintext modulus: {}, Moduli: {:?}", degree, plaintext_modulus, moduli);

    let _params = BfvParametersBuilder::new()
        .set_degree(degree as usize)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc().unwrap();

    // write public output to the journal
    env::commit(&degree);
}
