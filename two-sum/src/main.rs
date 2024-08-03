use std::collections::{HashMap, HashSet};

// leetcode
// https://leetcode.com/problems/two-sum/description/
//
// neetcode
// https://neetcode.io/problems/two-integer-sum
//

fn main() {
    two_sum_original(vec![2,7,11,15], 9);
    two_sum_optimized(vec![2,7,11,15], 9);

    two_sum_original(vec![3,2,4], 6);
    two_sum_optimized(vec![3,2,4], 6);
    
    two_sum_original(vec![3,3], 6);
    two_sum_optimized(vec![3,3], 6);
}

fn two_sum_original(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashSet::new();

    for (i, num) in nums.iter().enumerate() {
        if seen.contains(&num) {
            continue;
        }

        for (j, other_num) in nums[i+1..].iter().enumerate() {
            if num + other_num == target {
                println!("The sum combo of {:?} for {} is {:?} with indexes {:?}", nums, target, vec![*num, *other_num], vec![i, i+1+j]);
                return vec![i as i32, (i+1+j) as i32];
            }
        }

        seen.insert(num);
    }     

    panic!("You told me there'd be a result :(");
}

fn two_sum_optimized(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut targets: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if targets.contains_key(num) {
            println!("The sum combo of {:?} for {} is {:?}", nums, target,vec![*targets.get(num).unwrap(), i as i32]);

            return vec![*targets.get(num).unwrap(), i as i32];
        }

        targets.insert(target - num, i as i32);
    }     

    panic!("You told me there'd be a result :(");
}