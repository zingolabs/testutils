use std::fmt::Display;
pub struct Comparator<T: PartialEq + Display> {
    expected: T,
    observed: T,
}
pub impl<T: PartialEq + Display> Comparator<T> {
    pub fn compare(self) {
        //! Our convention is that "expected" is "first"
        if self.expected != self.observed {
            panic!(
                "\n===\nExpected:\n{}\nObserved:\n{}\n===\n",
                self.expected, self.observed
            );
        }
    }
}
