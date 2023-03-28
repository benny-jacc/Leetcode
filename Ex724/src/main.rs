fn main() {
    let result: i32 = pivot_index(vec![1, 7, 3, 6, 5, 6]);
    println!("{}", result);
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let pivot_ind: i32 = -1;
    
    for (ind, _num) in nums.iter().enumerate() {
        let nums_split: (&[i32], &[i32]) = nums.split_at(ind);

        let left_split = nums_split.0.iter();
        let mut right_split = nums_split.1.to_vec();

        right_split.remove(0);

        let left_sum: i32 = left_split.sum();
        let right_sum: i32 = right_split.iter().sum();

        if left_sum == right_sum { return ind as i32; } else { continue }
    }

    pivot_ind
}