#![allow(unused_imports)]
#![allow(unused_variables)]

use rand::thread_rng;

use ff::{BitIterator, Field, PrimeField, PrimeFieldRepr};

use pairing::Engine;

// We're going to use the BLS12-381 pairing-friendly elliptic curve.
use pairing::bls12_381::{Bls12, Fr};

use sapling_crypto::circuit::{blake2s, boolean, ecc, multipack, num, Assignment};

use sapling_crypto::circuit::boolean::{AllocatedBit, Boolean};

use bellman::{Circuit, ConstraintSystem, SynthesisError};

// We're going to use the Groth16 proving system.
use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
};

// We have some dummy input variable.
pub struct Blake2sBench<E: Engine> {
    pub num_bytes: i32,
    pub dummy: Option<E::Fr>,
}

impl<E: Engine> Circuit<E> for Blake2sBench<E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        use rand::{Rng, SeedableRng, XorShiftRng};
        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

        // Allocate the dummy value
        let a = cs.alloc_input(|| "dummy", || {
            self.dummy.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let input_len = self.num_bytes;
        let data: Vec<u8> = (0..input_len).map(|_| rng.gen()).collect();
        println!("#Number of bytes: {}", data.len());

        let mut input_bits = vec![];

        for (byte_i, input_byte) in data.into_iter().enumerate() {
            for bit_i in 0..8 {
                let cs = cs.namespace(|| format!("input bit {} {}", byte_i, bit_i));

                input_bits.push(
                    AllocatedBit::alloc(cs, Some((input_byte >> bit_i) & 1u8 == 1u8))
                        .unwrap()
                        .into(),
                );
            }
        }

        let hash = blake2s::blake2s(
            cs.namespace(|| "calculate hash"),
            &input_bits,
            b"12345678",
        )?;

        Ok(())
    }
}
