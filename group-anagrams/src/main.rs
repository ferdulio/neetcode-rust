use std::collections::HashMap;

// leetcode
// https://leetcode.com/problems/group-anagrams/
//
// neetcode
// https://neetcode.io/problems/anagram-groups
//

fn main() {
    group_anagrams(vec![
        "eat".to_owned(),
        "tea".to_owned(),
        "tan".to_owned(),
        "ate".to_owned(),
        "nat".to_owned(),
        "bat".to_owned(),
    ]);
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();

    for entry in strs.iter() {
        let mut sorted: Vec<char> = entry.chars().collect();
        sorted.sort_unstable();

        let key = String::from_iter(sorted.iter());
        
        if hash_map.contains_key(&key) {
            let list = hash_map.get_mut(&key).unwrap();
            list.push(entry.to_owned());
        } else {
            hash_map.insert(key, vec![entry.to_owned()]);
        }
    }

    let result2: Vec<Vec<String>> = hash_map.into_values().collect();
    println!("Result: {:?}", result2);

    result2
}
