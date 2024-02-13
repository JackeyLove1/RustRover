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
        for j in i+1..arr.len(){
            if (arr[i] > arr[j]){
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
    use super::*;


}