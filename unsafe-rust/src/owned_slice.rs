pub struct OwnedSlice<T: 'static> {
    vec: Vec<T>,
    slice: &'static [T],
}

impl<T: 'static> OwnedSlice<T> {
    pub fn from_vec(vec: Vec<T>, range: std::ops::Range<usize>) -> Self {
        let slice_ref = vec.get(range).unwrap();
        let slice_ptr = slice_ref as *const [T];
        let slice: &'static [T] = unsafe { &*slice_ptr };

        Self { vec, slice }
    }

    pub fn slice(&self) -> &[T] {
        self.slice
    }
}
