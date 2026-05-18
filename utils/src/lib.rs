pub mod inputs;

pub fn boxed_grid<T: Clone>(height: usize, width: usize, val: T) -> Box<[Box<[T]>]> {
    std::iter::repeat_with(|| vec![val.clone(); width].into_boxed_slice())
        .take(height)
        .collect()
}
