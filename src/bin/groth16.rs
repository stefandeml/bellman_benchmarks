#![allow(unused_imports)]
#![allow(unused_variables)]
use std::env;

extern crate bellman;
extern crate benchmarks;
//extern crate pairing;
use bellman::pairing;
extern crate rand;
use benchmarks::circuit::Blake2sBench;
use rand::{Rng, SeedableRng, XorShiftRng};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_bytes = args[1].clone().parse().unwrap();

    use bellman::groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    };
//    use ff::Field;
//    use ff::PrimeField;
    use bellman::pairing::ff::PrimeField;
    use bellman::pairing::ff::Field;
    use pairing::bls12_381::{Bls12, Fr};
    use rand::thread_rng;

    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let start = std::time::Instant::now();
    println!("Creating parameters...");

    // Create parameters for our circuit
    let params = {
        let c = Blake2sBench::<Bls12> {
            num_bytes,
            dummy: None,
        };

        generate_random_parameters(c, &mut rng).unwrap()
    };

    let buffer = File::create(format!("params_{}.txt", num_bytes)).unwrap();
    let _ = params.write(buffer);

    // Prepare the verification key (for proof verification)
    println!(
        "#Parameter generation takes seconds: {}",
        start.elapsed().as_millis() as f64 / 1000.0
    );
    let start = std::time::Instant::now();

    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");

    let dummy = Fr::from_str("21");
    // Create an instance of circuit
    let c = Blake2sBench::<Bls12> {
        num_bytes,
        dummy: dummy,
    };

    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, &mut rng).unwrap();

    assert!(verify_proof(&pvk, &proof, &[dummy.unwrap()]).unwrap());

    println!(
        "#Proof generation takes seconds: {}",
        start.elapsed().as_millis() as f64 / 1000.0
    );
}
