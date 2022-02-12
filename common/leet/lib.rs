#[macro_export]
macro_rules! setup {
    ( $main_name:ident, tests = [
        $( ( $test_name:ident, given = ($($input:expr),*), should_return = $output:expr ) ),* $(,)?
    ] ) => {
        pub struct Solution;

        #[cfg(test)]
        mod tests {
            use super::*;
            $(
                #[test]
                fn $test_name() {
                    let output = Solution::$main_name($($input,)*);
                    assert_eq!(output, $output);
                }
            )*
        }
    };
}
