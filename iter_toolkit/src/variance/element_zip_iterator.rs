#[derive(Clone)]
pub struct ElementZipIterator<I1, I2, IS>
where
    I1: Iterator<Item = IS>,
    I2: Iterator + Clone,
    IS: IntoIterator,
{
    it1: I1,
    it2: I2,
}

impl<I1, I2, IS> ElementZipIterator<I1, I2, IS>
where
    I1: Iterator<Item = IS>,
    I2: Iterator + Clone,
    IS: IntoIterator,
{
    pub fn make(it1: I1, it2: I2) -> Self {
        ElementZipIterator { it1, it2 }
    }
}

impl<I1, I2, IS> Iterator for ElementZipIterator<I1, I2, IS>
where
    I1: Iterator<Item = IS>,
    I2: Iterator + Clone,
    IS: IntoIterator,
{
    type Item = Vec<(IS::Item, I2::Item)>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.it1.next() {
            Some(e.into_iter().zip(self.it2.clone().into_iter()).collect())
        } else {
            None
        }
    }
}
