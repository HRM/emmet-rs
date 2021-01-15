use variance_iterator::VarianceIterator;
use crate::variance::element_zip_iterator::ElementZipIterator;
use crate::variance::pair_iterator::PairIterator;
use crate::variance::variance_iterator_if::VarianceIteratorFiltered;
use crate::variance::partial_pair_iterator::PartialPairIterator;

mod element_zip_iterator;
mod helper;
mod pair_iterator;
mod variance_iterator;
mod variance_iterator_if;
mod partial_pair_iterator;

pub trait VariableIter: Iterator {
    fn variate(self, n: usize) -> VarianceIterator<Self::Item>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        VarianceIterator::make(self, n)
    }

    fn variate_if<C: FnMut(usize, Self::Item) -> bool>(
        self,
        n: usize,
        filter: C,
    ) -> VarianceIteratorFiltered<Self::Item, C>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        VarianceIteratorFiltered::make(self, filter, n)
    }

    fn pair<U: Clone, C: FnMut(Self::Item, U) -> bool, It: Iterator<Item = U> + Sized>(
        self,
        it: It,
        filter: C,
    ) -> PairIterator<Self::Item, U, C>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        PairIterator::make(self, it, filter)
    }

    fn pair_partial<U: Clone, C: FnMut(Self::Item, U) -> bool, It: Iterator<Item = U> + Sized>(
        self,
        it: It,
        filter: C,
    ) -> PartialPairIterator<Self::Item, U, C>
        where
            Self: Sized,
            Self::Item: Clone,
    {
        PartialPairIterator::make(self, it, filter)
    }

    fn zip_elements<I2>(self, it: I2) -> ElementZipIterator<Self, I2, Self::Item>
    where
        Self: Sized,
        I2: Iterator + Clone,
        Self::Item: IntoIterator,
    {
        ElementZipIterator::make(self, it)
    }
}

impl<T: ?Sized> VariableIter for T where T: Iterator {}
