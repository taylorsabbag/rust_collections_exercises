use std::collections::HashMap;

pub fn median(nums: &[i32]) -> Option<i32> {
    let length = nums.len();
    let middle_num_idx = length / 2;
    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort_unstable();
    sorted_nums.get(middle_num_idx).copied()
}

pub fn mode(nums: &[i32]) -> Option<i32> {
    let mut counts = HashMap::new();
    for &num in nums {
        counts
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
}
