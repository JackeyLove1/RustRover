#[cfg(test)]
mod tests{
    use std::cmp::Ordering;
    use tracing_subscriber::filter::combinator::Or;

    #[test]
    fn test_ord(){
        use std::cmp::Ordering;
        assert_eq!(5.cmp(&10), Ordering::Less);
        assert_eq!(5.cmp(&5), Ordering::Equal);
        assert_eq!(5.cmp(&1), Ordering::Greater);
    }
}