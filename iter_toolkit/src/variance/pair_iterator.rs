use crate::variance::helper::{push_stack, rebuild_stack};
use std::cmp::max;
use std::{cmp::min, iter::FusedIterator};

#[derive(Clone)]
pub struct PairIterator<T: Clone, U: Clone, C: FnMut(T, U) -> bool> {
    indices: Vec<usize>,
    collection: Vec<T>,
    collection2: Vec<U>,
    ended: bool,
    if_func: C,
}

impl<T: Clone, U: Clone, C: FnMut(T, U) -> bool> PairIterator<T, U, C> {
    pub fn make<F: Iterator<Item = T>, F2: Iterator<Item = U>>(
        iter1: F,
        iter2: F2,
        filter: C,
    ) -> PairIterator<F::Item, F2::Item, C> {
        let collection = iter1.collect::<Vec<F::Item>>();
        let collection2 = iter2.collect::<Vec<F2::Item>>();
        let ended: bool = false;

        PairIterator {
            indices: Vec::new(),
            collection,
            collection2,
            ended,
            if_func: filter,
        }
    }
}

impl<T: Clone, U: Clone, C: FnMut(T, U) -> bool> FusedIterator for PairIterator<T, U, C> {}

impl<T: Clone, U: Clone, C: FnMut(T, U) -> bool> Iterator for PairIterator<T, U, C> {
    type Item = Vec<(T, U)>;

    fn next(&mut self) -> Option<Vec<(T, U)>> {
        if self.ended {
            return None;
        }

        if self.collection.is_empty() || self.collection2.is_empty() {
            self.ended = true;
            return Some(Vec::new());
        }

        let swapped = self.collection2.len() > self.collection.len();
        let min = min(self.collection.len(), self.collection2.len());
        let max = max(self.collection.len(), self.collection2.len());

        let mut stack = rebuild_stack(max, min, &mut self.indices);

        loop {
            if *(self.indices.last().unwrap()) == stack.last().unwrap().len() {
                stack.pop();
                self.indices.pop();
                if self.indices.is_empty() {
                    self.ended = true;
                    if self.collection.is_empty() || self.collection2.is_empty() {
                        return Some(Vec::new());
                    }
                    return None;
                } else {
                    *(self.indices.last_mut().unwrap()) += 1_usize;
                }
            } else {
                let v1: T;
                let v2: U;

                let pos = self.indices.len() - 1;

                if swapped {
                    v1 = self.collection[pos].clone();
                    v2 = self.collection2[stack[pos][self.indices[pos]]].clone();
                } else {
                    v1 = self.collection[stack[pos][self.indices[pos]]].clone();
                    v2 = self.collection2[pos].clone();
                }

                if !(self.if_func)(v1, v2) {
                    *(self.indices.last_mut().unwrap()) += 1_usize;
                    continue;
                }

                if self.indices.len() == min {
                    break;
                }

                push_stack(&mut self.indices, &mut stack);
            }
        }
        let mut ret: Vec<(T, U)> = Vec::with_capacity(min);
        for i in 0..min {
            let ind = stack[i][self.indices[i]];
            if swapped {
                ret.push((self.collection[i].clone(), self.collection2[ind].clone()));
            } else {
                ret.push((self.collection[ind].clone(), self.collection2[i].clone()));
            }
        }
        Some(ret)
    }
}
