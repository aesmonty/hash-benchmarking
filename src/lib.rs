use blake2::{Blake2b, Blake2s};
use blake3::Hasher;
use sha3::{Digest, Keccak256};

pub const TIME: u128 = 1625636850;
pub const UPPER: u128 = 100;

pub fn my_sha3_hash(val: u128) -> u128 {
    let mut hasher = Keccak256::new();
    hasher.update(val.to_string());
    let mut dst: [u8; 16] = [0u8; 16];
    dst.clone_from_slice(&hasher.finalize()[..16]);

    u128::from_be_bytes(dst)
}

pub fn my_blake2s_hash(val: u128) -> u128 {
    let mut hasher = Blake2s::new();
    hasher.update(val.to_string());
    let mut dst: [u8; 16] = [0u8; 16];
    dst.clone_from_slice(&hasher.finalize()[..16]);

    u128::from_be_bytes(dst)
}

pub fn my_blake2b_hash(val: u128) -> u128 {
    let mut hasher = Blake2b::new();
    hasher.update(val.to_string());
    let mut dst: [u8; 16] = [0u8; 16];
    dst.clone_from_slice(&hasher.finalize()[..16]);

    u128::from_be_bytes(dst)
}

pub fn my_blake3_hash(val: u128) -> u128 {
    let mut hasher = Hasher::new();
    hasher.update(val.to_string().as_ref());
    let mut dst: [u8; 16] = [0u8; 16];
    dst.clone_from_slice(&hasher.finalize()[..16]);

    u128::from_be_bytes(dst)
}

pub fn rounding_algo(entropy: u128, upperbound: u128, hash: fn(u128) -> u128) -> u128 {
    let min: u128 = (u128::MAX - upperbound) % upperbound;
    let mut random: u128 = hash(entropy);

    loop {
        if random >= min {
            break;
        }
        random = hash(random);
    }

    random % upperbound
}
