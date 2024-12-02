pub struct SkipOne<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    skip: usize,
    current: usize,
}

impl<I, T> Iterator for SkipOne<I, T>
where
    I: Iterator<Item = T>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.skip {
            self.iter.next();
        }
        self.current += 1;
        self.iter.next()
    }
}

pub trait SkipOneIter<T>
where
    Self: Sized + Iterator<Item = T>,
{
    fn skip_one(self, i: usize) -> SkipOne<Self, T>;
}

impl<I, T> SkipOneIter<T> for I
where
    I: Iterator<Item = T>,
{
    fn skip_one(self, i: usize) -> SkipOne<I, T> {
        SkipOne {
            iter: self,
            skip: i,
            current: 0,
        }
    }
}
