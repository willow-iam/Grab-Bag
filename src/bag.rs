
pub trait Bag<T> {
    fn push(&mut self,item:T);
    fn pop(&mut self)->Option<T>;
    fn new<I:Iterator<Item=T>>(items:I)-> Self;
}
