pub trait Sorter<T: Ord + Copy> {
    fn sort_inplace(arr: &mut [T]);

    fn sort(arr: &[T]) -> Vec<T>{
        let mut arr = arr.to_vec();
        Self::sort_inplace(&mut arr);
        return arr;

    }
}

fn bubble_sort<T: Ord>(arr: &mut [T]){
    for i in 0..arr.len(){
        for j in (i+1)..arr.len(){
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort
where 
    T: Ord + Copy 
{
    fn sort_inplace(arr: &mut [T]) {
        bubble_sort(arr);
    }
}

#[cfg(test)]
mod tests{
    use tracing_subscriber::fmt::writer::EitherWriter::B;
    use crate::practice::generate_random_vec;
    use super::*;
    #[test]
    fn test_is_sorted(){
        let mut vec1 = vec![3,1,4,-1,2,3,5,6,7,8,10];
        let mut vec2 = vec1.clone();
        vec2.sort();
        BubbleSort::sort_inplace(&mut vec1);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn test_random_sorted(){
        let size : usize = 100;
        let mut vec1 : Vec<i32> = generate_random_vec(size);
        let mut vec2 = vec1.clone();
        vec2.sort();
        BubbleSort::sort_inplace(&mut vec1);
        assert_eq!(vec1, vec2);
    }
}