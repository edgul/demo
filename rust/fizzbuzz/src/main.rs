fn fizzbuzz(i: u32) -> String {
    if i == 0 {
        return "0".to_string();
    }

    let mut result = "".to_string();
    if i % 3 == 0 {
        result.push_str("Fizz");
    }
    if i % 5 == 0 {
        result.push_str("Buzz");
    }
    if result.is_empty() {
        return format!("{}", i);
    }
    result
}

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(0), "0".to_string());
    assert_eq!(fizzbuzz(1), "1".to_string());
    assert_eq!(fizzbuzz(2), "2".to_string());
    assert_eq!(fizzbuzz(3), "Fizz".to_string());
    assert_eq!(fizzbuzz(4), "4".to_string());
    assert_eq!(fizzbuzz(5), "Buzz".to_string());
    assert_eq!(fizzbuzz(6), "Fizz".to_string());
    assert_eq!(fizzbuzz(7), "7".to_string());
    assert_eq!(fizzbuzz(8), "8".to_string());
    assert_eq!(fizzbuzz(9), "Fizz".to_string());

    assert_eq!(fizzbuzz(10), "Buzz".to_string());
    assert_eq!(fizzbuzz(11), "11".to_string());
    assert_eq!(fizzbuzz(12), "Fizz".to_string());
    assert_eq!(fizzbuzz(13), "13".to_string());
    assert_eq!(fizzbuzz(14), "14".to_string());
    assert_eq!(fizzbuzz(15), "FizzBuzz".to_string());
    assert_eq!(fizzbuzz(16), "16".to_string());
    assert_eq!(fizzbuzz(17), "17".to_string());
    assert_eq!(fizzbuzz(18), "Fizz".to_string());
    assert_eq!(fizzbuzz(19), "19".to_string());
}

fn fizzbuzz_match(i: u32) -> String {
    if i == 0 {
        return "0".to_string();
    }

    let mod_3 = |i: u32| { 
        i%3==0 
    };
    let mod_5 = |i: u32| {
        i%5==0
    };

    match (mod_3(i), mod_5(i)) {
      (true, true) => return "FizzBuzz".to_string(),
      (true, _)    => return "Fizz".to_string(),
      (_, true)    => return "Buzz".to_string(),
      _                => return format!("{}", i)
    }
}
#[test]
fn test_fizzbuzz_match() {
    assert_eq!(fizzbuzz_match(0), "0".to_string());
    assert_eq!(fizzbuzz_match(1), "1".to_string());
    assert_eq!(fizzbuzz_match(2), "2".to_string());
    assert_eq!(fizzbuzz_match(3), "Fizz".to_string());
    assert_eq!(fizzbuzz_match(4), "4".to_string());
    assert_eq!(fizzbuzz_match(5), "Buzz".to_string());
    assert_eq!(fizzbuzz_match(6), "Fizz".to_string());
    assert_eq!(fizzbuzz_match(7), "7".to_string());
    assert_eq!(fizzbuzz_match(8), "8".to_string());
    assert_eq!(fizzbuzz_match(9), "Fizz".to_string());
    assert_eq!(fizzbuzz_match(10), "Buzz".to_string());
    assert_eq!(fizzbuzz_match(11), "11".to_string());
    assert_eq!(fizzbuzz_match(12), "Fizz".to_string());
    assert_eq!(fizzbuzz_match(13), "13".to_string());
    assert_eq!(fizzbuzz_match(14), "14".to_string());
    assert_eq!(fizzbuzz_match(15), "FizzBuzz".to_string());
    assert_eq!(fizzbuzz_match(16), "16".to_string());
    assert_eq!(fizzbuzz_match(17), "17".to_string());
    assert_eq!(fizzbuzz_match(18), "Fizz".to_string());
    assert_eq!(fizzbuzz_match(19), "19".to_string());
}

fn main() {
    for i in 0..50 {
        println!("fizzbuzz({}): {}", i, fizzbuzz(i));
    }
}
