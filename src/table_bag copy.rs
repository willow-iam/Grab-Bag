use rand::{distributions::{uniform::{SampleBorrow, SampleUniform}, WeightedIndex}, Rng, SeedableRng};
use rand_pcg::Pcg64Mcg as pcg;
use crate::bag::Bag;
use std::{collections::HashMap, hash::Hash, iter::Sum, ops::AddAssign};
use num_traits::{NumAssignOps, One, Zero};

trait Counter : 
    AddAssign+Sum+Zero+One+PartialOrd+
    Clone+Default+SampleUniform{}

pub struct TableBag<T:Eq+Hash,R:SeedableRng>{
    items: HashMap<T,u32>,
    rng:R,
    
}
impl<T:Eq+Hash,R:SeedableRng> TableBag<T,R>{
    fn count(&self) -> u32{
        self.items.values().cloned().sum()
    }
}
impl<T:Eq+Hash,R:SeedableRng+Rng> Bag<T> for TableBag<T,R,C>{
    fn push(&mut self,item:T) {
        *self.items.entry(item).or_insert(C::zero()) += C::one();
    }

    fn pop(&mut self)->Option<T> {
        let choices=self.items.values().clone();
        WeightedIndex::new(choices.clone()).sample(self.rng)
    }

    fn new<I:Iterator<Item=T>>(items:I)-> Self {
        let mut rax = TableBag{
            items:HashMap::new(),
            rng:R::from_entropy()
        };
        for item in items{
            rax.push(item);
        }
        rax
    }
}