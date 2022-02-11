use std::cmp::Ordering;

/// Performs binary search and returns the index of the found element.
pub fn bs_index<T: Ord>(xs: &[T], x: &T) -> Option<usize> {
    let mut upper = xs.len();
    let mut lower = 0;

    while lower < upper {
        let index = lower + (upper - lower) / 2;
        let curr = &xs[index];

        match curr.cmp(x) {
            Ordering::Less => lower = index + 1,
            Ordering::Greater => upper = index,

            Ordering::Equal => return Some(index),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::bs_index;

    const LIST: &[usize] = &[1, 3, 5, 7, 9, 11, 13, 15];

    #[test]
    fn test() {
        for i in 0usize..=16 {
            let maybe_index = bs_index(LIST, &i);
            if i % 2 == 0 {
                assert_eq!(maybe_index, None);
                assert!(!LIST.contains(&i));
            } else {
                let index = maybe_index.expect("Expected `Some`.");
                assert_eq!(index, i / 2);
                assert_eq!(LIST[index], i);
            }
        }
    }
}
