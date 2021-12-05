pub trait Arr<T, const N: usize>: Iterator<Item = T> + Sized {
    fn try_arr(self) -> Option<[T; N]>;

    fn arr(self) -> [T; N] {
        self.try_arr().unwrap()
    }
}

impl<I: Iterator<Item = T>, T, const N: usize> Arr<T, N> for I {
    fn try_arr(self) -> Option<[T; N]> {
        use std::mem::{self, MaybeUninit};

        let mut iter = self.into_iter();

        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
        // safe because the type we are claiming to have initialized here is a
        // bunch of `MaybeUninit`s, which do not require initialization.
        let mut data: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
        // assignment instead of `ptr::write` does not cause the old
        // uninitialized value to be dropped. Also if there is a panic during
        // this loop, we have a memory leak, but there is no memory safety
        // issue.
        for elem in &mut data[..] {
            if let Some(val) = iter.next() {
                elem.write(val);
            } else {
                return None;
            }
        }

        Some(
            // Everything is initialized. Transmute the array to the
            // initialized type.
            unsafe { mem::transmute_copy::<_, [T; N]>(&data) },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Arr;

    #[test]
    fn test_1() {
        let arr: [char; 3] = "abc".chars().arr();
        assert_eq!(arr, ['a', 'b', 'c']);
    }

    #[test]
    fn test_2() {
        let [a, b] = "abc".chars().arr();
        assert_eq!(a, 'a');
        assert_eq!(b, 'b');
    }

    #[test]
    fn test_3() {
        let arr: Option<[char; 4]> = "abc".chars().try_arr();
        assert_eq!(arr, None);
    }

    #[test]
    #[should_panic]
    fn test_4() {
        let _: [char; 4] = "abc".chars().arr();
    }
}
