use std::ops::Deref;


struct MyBox<T> (T);

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl <T> MyBox<T> {
    fn new(x : T) -> MyBox<T> {
        return MyBox(x);
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_deref(){
        let x = 1;
        let y = MyBox::new(x);
        assert_eq!(x, *y);
    }

}