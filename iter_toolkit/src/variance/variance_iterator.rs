use crate::variance::helper::{push_stack, rebuild_stack};

#[derive(Clone)]
pub struct VarianceIterator<T: Clone> {
    indices: Vec<usize>,
    collection: Vec<T>,
    n: usize,
    ended: bool,
}

impl<T: Clone> VarianceIterator<T> {
    pub fn make<F: Iterator<Item = T>>(iter: F, n: usize) -> VarianceIterator<F::Item> {
        let collection = iter.collect::<Vec<F::Item>>();
        let ended: bool = n > collection.len();
        VarianceIterator {
            indices: Vec::new(),
            collection,
            n,
            ended,
        }
    }
}


impl<T: Clone> Iterator for VarianceIterator<T> {
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
