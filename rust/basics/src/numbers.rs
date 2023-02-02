use std::collections::HashMap;

#[allow(dead_code)]
pub fn even(number: i32) -> bool {
    number % 2 == 0
}

#[test] 
fn test_even() {

    // natural numbers
    assert!(even(0));
    assert!(!even(1));
    assert!(even(2));
    assert!(!even(3));
    
    // negative
    assert!(!even(-1));
    assert!(even(-2));
}


// Requirements:
// * returns indices of two numbers that add to target
// * may not use same element twice
// Assumes:
// * exactly one solution
// * answer can be in any order
// Observations:
// * low memory use
// * O(n^2) runtime complexity
// * Approach is more easily extendible to return multiple solutions  
#[allow(dead_code)]
pub fn two_sum_slow(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i,vi) in nums.iter().enumerate() {
       for (j,vj) in nums.iter().enumerate() {
            if i != j {
                if vi + vj == target {
                    return vec!(i as i32,j as i32);
                }
            }
        }
    }
    vec!(-1,-1)
}

// Same requirements and assumptions as above
// Observations:
// * O(n) runtime complexity (fast)
// * heavy mem consumption O(n)
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i,vi) in nums.iter().enumerate() {
        let answer = target - vi;
        if map.contains_key(&answer) {
            let answer_index = map[&answer];
            if i < answer_index { // also sort the answer
                return vec!(i as i32, answer_index as i32);
            }
            return vec!(answer_index as i32, i as i32);
        }

        map.insert(vi, i); 
    }
    vec!(-1,-1)
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum_slow(vec!(1,2,3,4),4), vec!(0,2));
    assert_eq!(two_sum_slow(vec!(1,2,3,6),5), vec!(1,2));

    assert_eq!(two_sum(vec!(1,2,3,4),4), vec!(0,2));
    assert_eq!(two_sum(vec!(1,2,3,6),5), vec!(1,2));
}
