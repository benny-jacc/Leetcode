fn main() {
    let vector: Vec<i32> = running_sum(vec![1, 2, 3, 4]);
    println!("${:?}", vector);
}

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut running_sum: Vec<i32> = Vec::new();

    for (ind, _num) in nums.iter().enumerate() {
        running_sum.push(
            nums.split_at(ind + 1).0.iter().sum()
        );
    }

    running_sum
}