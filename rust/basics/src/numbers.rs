
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

// returns indices of two numbers that add to target
// assumes 
// * exactly one solution
// * may not use same element twice
// * answer can be in any order
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec!(1,2,3,4),4), vec!(0,2));
    assert_eq!(two_sum(vec!(1,2,3,6),5), vec!(1,2));
}
