extern crate rand;

use rand::Rng;

pub fn generate_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(len)
        .collect()
}
