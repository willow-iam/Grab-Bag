

use rand::{Rng,SeedableRng};
use crate::bag::Bag;


pub struct ArrayBag<T,R:SeedableRng>{
    items : Vec<T>,
    rng : R,
}

impl<T,R:SeedableRng+Rng> Bag<T> for ArrayBag<T,R>{
    fn push(&mut self,item:T) {
        self.items.push(item)
    }

    fn pop(&mut self)->Option<T> {
        if self.items.is_empty() { None }
        else { Some(self.items.swap_remove(self.rng.gen_range(0..self.items.len()))) }
    }
    
    fn new<I:Iterator<Item=T>>(items:I)-> Self{
        ArrayBag { items:Vec::from_iter(items), rng:R::from_entropy()}
    }
}
