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

    use bellman::pairing::ff::PrimeField;
    use bellman::pairing::ff::Field;
    use rand::thread_rng;
    use bellman::sonic::helped::{
        generate_random_parameters,
        verify_aggregate,
        verify_proofs,
        create_proof,
        create_advice,
        create_aggregate,
        get_circuit_parameters
    };
    use pairing::bn256::{Bn256, Fr};
    use std::time::{Instant};
    let samples: usize = 3;

    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    // dummy hash image
    let dummy = Fr::from_str("21");
    // Create an instance of circuit
    let circuit = Blake2sBench::<Bn256> {
        num_bytes,
        dummy: dummy,
    };

    let start = Instant::now();
    println!("Creating parameters...");
    let info = get_circuit_parameters::<Bn256, _>(circuit.clone()).expect("Must get circuit info");
    println!("{:?}", info);
    let params = generate_random_parameters(circuit.clone(), &mut rng).unwrap();
    println!("done in {:?}", start.elapsed());

    println!("creating proof");
    let start = Instant::now();
    let proof = create_proof(circuit.clone(), &params).unwrap();
    println!("done in {:?}", start.elapsed());

    let input_vec = vec![vec![dummy.unwrap()]];
    println!("verifying 1 proof without advice");
    let start = Instant::now();
    assert_eq!(verify_proofs(&vec![proof.clone()], &input_vec[..], circuit.clone(),
                             rng, &params).unwrap(), true);
    println!("done in {:?}", start.elapsed());

    println!("creating advice for circuit");
    let start = Instant::now();
    let advice = create_advice(circuit.clone(), &proof, &params).unwrap();
    println!("done in {:?}", start.elapsed());

    let input_vec = vec![vec![dummy.unwrap()]; samples];
    println!("creating aggregate for {} proofs", samples);
    let start = Instant::now();
    let proofs: Vec<_> = (0..samples).map(|_| (proof.clone(), advice.clone())).collect();
    let aggregate = create_aggregate::<Bn256, _>(circuit.clone(), &proofs, &params);
    println!("done in {:?}", start.elapsed());

    {
        println!("verifying {} proofs without advice", samples);
        let rng = thread_rng();
        let start = Instant::now();
        assert_eq!(verify_proofs(&vec![proof.clone(); samples], &input_vec, circuit.clone(), rng,
                                 &params).unwrap(), true);
        println!("done in {:?}", start.elapsed());
    }

    {
        println!("verifying {} proofs with advice and aggregate", samples);
        let rng = thread_rng();
        let start = Instant::now();
        assert_eq!(verify_aggregate(&vec![(proof.clone(), advice.clone()); samples], &aggregate,
                                    &input_vec, circuit.clone(), rng, &params).unwrap(), true);
        println!("done in {:?}", start.elapsed());
    }
}
