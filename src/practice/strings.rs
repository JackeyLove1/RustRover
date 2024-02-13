pub fn reverse(s : &str) -> String{
    return s.chars().rev().collect::<String>();
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_reverse_simple(){
        assert_eq!(reverse("hello"), String::from("olleh"));
    }
}
