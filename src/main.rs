extern crate histogram;
extern crate rand;

use histogram::Histogram;
use std::convert::TryFrom;

use uniform_rand::{my_blake2b_hash, my_blake2s_hash, my_blake3_hash, my_sha3_hash, rounding_algo};

const TIME: u128 = 1625636850;

fn main() {
    let mut hist = Histogram::new();
    let mut number = TIME;
    let upper = 100u128;
    for _ in 0..10000 {
        number += 1;
        let hash = rounding_algo(number, upper, my_sha3_hash);
        hist.increment(u64::try_from(hash).unwrap()).unwrap();
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
