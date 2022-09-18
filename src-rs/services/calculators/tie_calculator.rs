use std::fmt::Debug;

pub fn calculate_tie_offset<T: Debug, F: Fn(&T, &T) -> bool>(
    drivers: &Vec<T>,
    baseline_index: usize,
    cmp: F,
) -> usize {
    if baseline_index == 0 {
        0
    } else {
        let baseline = drivers.get(baseline_index).unwrap();

        let mut next_index = baseline_index - 1;
        while let Some(next_comparison) = drivers.get(next_index) {
            if !cmp(baseline, next_comparison) {
                return baseline_index - next_index - 1;
            }
            next_index -= 1;
        }

        baseline_index
    }
}
