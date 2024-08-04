// leetcode link:
// https://leetcode.com/problems/top-k-frequent-elements/

use std::collections::HashMap;

fn main() {
    
    top_k_frequent(vec![4,1,-1,2,-1,2,3], 2);
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let number_frequencies: HashMap<i32, i32> = nums.iter().fold(
        HashMap::new(), 
        |mut freqs, num| {
            freqs
                .entry(*num)
                .and_modify(|freq| (*freq) += 1)
                .or_insert(1);

            freqs
        }
    );

    let mut freqs: Vec<(i32, i32)> = number_frequencies.into_iter().collect();
    freqs.sort_unstable_by(|a, b| (b.1).cmp(&(a.1)));

    let res = freqs.iter().take(k as usize).map(
        |freq| 
        freq.0
    ).collect();

    println!("Freqs are {:?}", freqs);
    println!("Result for {:?} and {} is {:?}", nums, k, res);
    res
}