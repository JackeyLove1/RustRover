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

    #[test]
    fn test_vec_iter(){
        let vec: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];

        // find
        let res1 = vec.iter().find(|&n| n % 2 == 0);
        assert_eq!(*res1.unwrap(), 2);

        // all
        let res2 = vec.iter().all(|&n| n < 100);
        assert_eq!(res2, true);

        // any
        let res2 = vec.iter().any(|&n| n > 0);
        assert_eq!(res2, true);
        
        // count
        let res3 = vec.iter().count();
        assert_eq!(res3, 10);

        // filter
        let res4: Vec<_> = vec.iter().filter(|&n| n % 2 == 0).collect();
        assert_eq!(res4, vec![&2, &4, &6, &8, &10]);

        // rev
        let res5: Vec<_> = vec.iter().rev().collect();
        assert_eq!(res5, vec![&10, &9, &8, &7, &6, &5, &4, &3, &2, &1]);

        // clone
        let res6 = vec.iter().clone();
        assert_eq!(res6, vec![1,2,3,4,5,6,7,8,9,10]);

        // max
        let res6 = vec.iter().max();
        assert_eq!(*res6.unwrap(), 10);

        // min
        let res7 = vec.iter().min();
        assert_eq!(*res7.unwrap(), 1);

        // sum
        let res8 = vec.iter().sum();
        assert_eq!(res8, 55);

    }
}
