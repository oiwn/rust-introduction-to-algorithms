/// Common functions to generate data for use in test cases
use rand::{distributions::uniform::SampleUniform, thread_rng, Rng};

/// generate short length random vector (size of 10)
pub fn short_random_vector<T>(min: T, max: T) -> Vec<T>
where
    T: SampleUniform + PartialOrd + Copy,
{
    let mut rng = thread_rng();
    let mut vec = Vec::with_capacity(10);
    let range = min..=max;
    for _ in 0..10 {
        vec.push(rng.gen_range(range.clone()));
    }
    vec
}

/// generate long length random vector (size of 1million)
pub fn long_random_vector<T>(min: T, max: T) -> Vec<T>
where
    T: SampleUniform + PartialOrd + Copy,
{
    let mut rng = thread_rng();
    let mut vec = Vec::with_capacity(1000);
    let range = min..=max;
    for _ in 0..1000 {
        vec.push(rng.gen_range(range.clone()));
    }
    vec
}

/// Return empty vector of type T
pub fn empty_vector<T>() -> Vec<T> {
    Vec::new()
}
