pub fn is_permutation<T>(slice: &[T]) -> bool
where
    T: Copy,
    usize: From<T>,
{
    let n = slice.len();

    // Use a boolean array to track which numbers we've seen
    let mut seen = vec![false; n];

    for val in slice {
        let idx = usize::from(*val);

        if idx >= n {
            return false;
        }

        if seen[idx] {
            // We've seen this number before, so it's not a permutation
            return false;
        }
        seen[idx] = true;
    }

    // If we get here, all numbers 0 to n-1 appeared exactly once
    true
}
