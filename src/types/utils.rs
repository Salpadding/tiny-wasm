pub(crate) trait VecUtils {
    type Item;

    fn self_copy(&mut self, src: usize, dst: usize, len: usize);
    fn fill_from(&mut self, src: usize, len: usize, elem: Self::Item);
}

impl<T> VecUtils for Vec<T>
    where
        T: Sized + Copy,
{
    type Item = T;

    fn self_copy(&mut self, src: usize, dst: usize, len: usize) {
        for i in 0..len {
            self[dst + i] = self[src + i];
        }
    }

    fn fill_from(&mut self, src: usize, len: usize, elem: T) {
        self[src..src + len].fill(elem);
    }
}
