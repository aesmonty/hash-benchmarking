use sha3::{Digest, Keccak256};
use blake2::{Blake2b, Blake2s};
use blake3::Hasher;

const HEIGHT: u128 = 4775448;
pub const TIME: u128 = 1625636850;

//const UPPER: Uint256 = Uint256::from(100u64);
pub const UPPER: u128 = 100;

fn my_sha3_hash(val: u128) -> u128 {
    let mut hasher = Keccak256::new();
    hasher.update(val.to_string());
    let mut dst: [u8; 16] = [0u8; 16];
    dst.clone_from_slice(&hasher.finalize()[..16]);

    u128::from_be_bytes(dst)
}

fn my_blake2_hash(val: u128) -> u128 {
    let mut hasher = Blake2s::new();
    hasher.update(val.to_string());
    let mut dst: [u8; 16] = [0u8; 16];
    dst.clone_from_slice(&hasher.finalize()[..16]);

    u128::from_be_bytes(dst)
}

fn my_blake3_hash(val: u128) -> u128 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(val.to_string().as_ref());
    let mut dst: [u8; 16] = [0u8; 16];
    dst.clone_from_slice(&hasher.finalize()[..16]);

    u128::from_be_bytes(dst)
}

pub fn rn_gen_blake(entropy:u128, upperbound:u128) -> u128 {
    let min: u128 = (u128::MAX - upperbound) % upperbound;
    let mut random: u128 = my_blake2_hash(entropy);

    loop {
        if random >= min {
            break;
        }
        random = my_blake2_hash(random);
    }

    random % upperbound
}

pub fn rn_gen_blake3(entropy:u128, upperbound:u128) -> u128 {
    let min: u128 = (u128::MAX - upperbound) % upperbound;
    let mut random: u128 = my_blake2_hash(entropy);

    loop {
        if random >= min {
            break;
        }
        random = my_blake3_hash(random);
    }

    random % upperbound
}

pub fn rn_gen(entropy:u128, upperbound:u128) -> u128 {
    let min: u128 = (u128::MAX - upperbound) % upperbound;
    let mut random: u128 = my_sha3_hash(entropy);

    loop {
        if random >= min {
            break;
        }
        random = my_sha3_hash(random);
    }

    random % upperbound
}
