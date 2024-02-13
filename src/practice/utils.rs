use rand::prelude::*;
use rand::distributions::Standard;

pub fn generate_random_vec<T>(size: usize) -> Vec<T>
    where Standard: rand::distributions::Distribution<T>
{
    (0..size).map(|_| rand::random::<T>()).collect()
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_generate_random_vec(){
        let random_u8s: Vec<u8> = generate_random_vec(10);
        println!("Random u8s: {:?}", random_u8s);

        // Generate a vector of random f64 values
        let random_f64s: Vec<f64> = generate_random_vec(10);
        println!("Random f64s: {:?}", random_f64s);
    }
}