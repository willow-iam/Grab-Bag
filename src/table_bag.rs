use rand::{distributions::{uniform::SampleUniform, Distribution, WeightedIndex,WeightedError}, Rng, SeedableRng};
use crate::bag::Bag;
use std::{collections::HashMap, hash::Hash, iter::Sum, ops::{AddAssign, SubAssign}};
use num_traits::{One, Zero};

pub trait Counter : 
    AddAssign+SubAssign+Sum+Zero+One+
    SampleUniform+PartialOrd+ for<'a> AddAssign<&'a Self>+
    Clone+Default+{}

pub trait Key :
    Eq+Hash+Clone{}

pub struct TableBag<T:Key,R:SeedableRng,C:Counter>{
    items: HashMap<T,C>,
    rng:R,
}

impl<T:Key,R:SeedableRng+Rng,C:Counter> Bag<T> for TableBag<T,R,C>{
    fn push(&mut self,item:T) {
        *self.items.entry(item).or_insert(C::zero()) += C::one();
    }

    fn pop(&mut self) -> Option<T> {
        let mut pairs: Vec<(&T, &mut C)> = self.items.iter_mut().collect();
        match WeightedIndex::new(pairs.iter().map(|item| item.1.clone())) {
            Ok(dist) => {
                let (rax, count) = 
                    pairs.get_mut(dist.sample(&mut self.rng))
                    .expect(
                        "The index comes from a distribution bounded by the size of pairs"
                    );
                **count -= C::one();
                return Some(rax.clone());
            }
            Err(_error @ ( 
                WeightedError::NoItem 
                | WeightedError::InvalidWeight 
                | WeightedError::AllWeightsZero )) => None,
            Err(_error @ WeightedError::TooMany) => unreachable!()
        }
    }

    fn new<I:Iterator<Item=T>>(items:I) -> Self {
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