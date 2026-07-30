use std::collections::HashMap;

fn main() {
    let integers = vec![1, 2, 3, 3, 3, 4, 5, 6, 6, 7];

    fn median(nums: &[i32]) -> Option<i32> {
        let length = nums.len();
        let middle_num_idx = length / 2;
        let mut sorted_nums = nums.to_vec();
        sorted_nums.sort_unstable();
        sorted_nums.get(middle_num_idx).copied()
    }

    fn mode(nums: &[i32]) -> Option<i32> {
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

    match median(&integers) {
        Some(med) => println!("The median is: {med}"),
        None => println!("The list was empty. No median can be found."),
    }

    match mode(&integers) {
        Some(mode) => println!("The mode is: {mode}"),
        None => println!("The list was empty. No mode can be found."),
    }
}
