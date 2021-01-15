use std::iter::FusedIterator;

use crate::variance::helper::{push_stack, rebuild_stack};

#[derive(Clone)]
pub struct VarianceIteratorFiltered<T: Clone, C: FnMut(usize, T) -> bool> {
    indices: Vec<usize>,
    collection: Vec<T>,
    n: usize,
    ended: bool,
    if_func: C,
}

impl<T: Clone, C: FnMut(usize, T) -> bool> VarianceIteratorFiltered<T, C> {
    pub fn make<F: Iterator<Item = T>>(
        iter: F,
        filter: C,
        n: usize,
    ) -> VarianceIteratorFiltered<F::Item, C> {
        let collection = iter.collect::<Vec<F::Item>>();
        let ended: bool = n > collection.len();
        VarianceIteratorFiltered {
            indices: Vec::new(),
            collection,
            n,
            ended,
            if_func: filter,
        }
    }
}

impl<T: Clone, C: FnMut(usize, T) -> bool> FusedIterator for VarianceIteratorFiltered<T, C> {}

impl<T: Clone, C: FnMut(usize, T) -> bool> Iterator for VarianceIteratorFiltered<T, C> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.ended {
            return None;
        }

        let mut stack = rebuild_stack(self.collection.len(), self.n, &mut self.indices);

        loop {
            if *(self.indices.last().unwrap()) == stack.last().unwrap().len() {
                stack.pop();
                self.indices.pop();
                if self.indices.is_empty() {
                    self.ended = true;
                    return None;
                } else {
                    *(self.indices.last_mut().unwrap()) += 1_usize;
                }
            } else {
                let pos = self.indices.len() - 1;
                if !(self.if_func)(pos, self.collection[stack[pos][self.indices[pos]]].clone()) {
                    *(self.indices.last_mut().unwrap()) += 1_usize;
                    continue;
                }

                if self.indices.len() == self.n {
                    break;
                }

                push_stack(&mut self.indices, &mut stack);
            }
        }
        let mut ret: Vec<T> = Vec::with_capacity(self.n);
        for i in 0..(self.n) {
            let ind = stack[i][self.indices[i]];
            ret.push(self.collection[ind].clone());
        }
        Some(ret)
    }
}
