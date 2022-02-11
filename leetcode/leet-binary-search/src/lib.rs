impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::binary_search_index(&nums, target)
            .map(|index| index as i32)
            .unwrap_or(-1)
    }

    /// Returns the index of `target`, performing binary search. Returns `None` if not found.
    pub fn binary_search_index<T: Ord>(slice: &[T], target: T) -> Option<usize> {
        Self::bsi_rec(0, slice, target)
    }

    fn bsi_rec<T: Ord>(slice_offset: usize, slice: &[T], target: T) -> Option<usize> {
        if slice.is_empty() {
            return None;
        }

        let i = slice.len() / 2;
        let middle_element = &slice[i];

        use std::cmp::Ordering::*;
        match middle_element.cmp(&target) {
            // If the "middle element" is greater, search in the lower group.
            Greater => Self::bsi_rec(slice_offset, &slice[..i], target),
            // If the "middle element" is less, search in the upper group.
            Less => Self::bsi_rec(slice_offset + i + 1, &slice[i + 1..], target),
            // If we found it, return.
            Equal => return Some(slice_offset + i),
        }
    }
}

leet::setup!(
    search,
    tests = [
        (
            case_found_1,
            given = (vec![-1, 0, 3, 5, 9, 12], 9),
            expects = 4
        ),
        (case_found_2, given = (vec![2, 5], 2), expects = 0),
        (
            case_not_found,
            given = (vec![-1, 0, 3, 5, 9, 12], 2),
            expects = -1
        ),
    ]
);
