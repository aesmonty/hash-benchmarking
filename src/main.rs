extern crate histogram;
extern crate rand;

use cosmwasm_bignumber::Uint256;
use histogram::Histogram;
use rand::Rng;
use std::convert::TryFrom;
use std::io;

use uniform_rand::{rn_gen, rn_gen_blake, rn_gen_blake3};

const HEIGHT: u128 = 4775448;
const TIME: u128 = 1625636850;

//const UPPER: Uint256 = Uint256::from(100u64);
const UPPER: u128 = 100;

fn main() {

    let mut hist = Histogram::new();
    let mut number = TIME;
    let upper = 100u128;
    for _ in 0..10000 {
        number += 1;
        let hash = rn_gen_blake3(number, upper);
        hist.increment(u64::try_from(hash).unwrap());
        //println!("{}", hash)
    }

    // print percentiles from the histogram
    println!(
        "Percentiles: p10: {} p25: {} ns p50: {} ns p90: {} ns p99: {}",
        hist.percentile(10.0).unwrap(),
        hist.percentile(25.0).unwrap(),
        hist.percentile(50.0).unwrap(),
        hist.percentile(90.0).unwrap(),
        hist.percentile(99.0).unwrap(),
    );

    // print additional statistics
    println!(
        "Latency (ns): Min: {} Avg: {} Max: {} StdDev: {}",
        hist.minimum().unwrap(),
        hist.mean().unwrap(),
        hist.maximum().unwrap(),
        hist.stddev().unwrap(),
    );
}


