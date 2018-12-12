#[derive(Debug)]
pub struct Array2<T> {
    inner: Vec<T>,
    shape: (usize, usize),
}
