
#[allow(dead_code)]
pub fn string_has_char(s: &str, c: char) -> bool {
    for i in s.chars() {
        if i == c {
            return true;
        }
    }
    false
}

#[test]
fn test_string_has_char() {
    assert!(string_has_char("some str", 's'));
    assert!(string_has_char("some str", 'r'));
    assert!(!string_has_char("some str", 'a'));
}

#[allow(dead_code)]
pub fn string_has_substr(s: &str, sub: &str) -> bool {
    // empty s can contain nothing
    if s.is_empty() {
        return false;
    }
    // s cannot contain longer sub
    if s.len() < sub.len() {
        return false;
    }
    // empty sub is always contained 
    if sub.is_empty() {
        return true;
    } 

    let s_vec = s.as_bytes().to_vec(); // convert to vector of chars
    for (i, _) in s_vec.iter().enumerate() { // get index
        let sub_vec = sub.as_bytes().to_vec(); 
       
        // walk  inner and outer vec together, without losing s index
        let mut same_so_far = true;
        for (j, sub_c) in sub_vec.iter().enumerate() {
            // if we fall off the end of s, we don't contain
            if i+j >= s_vec.len() { // bounds check
                return false;
            }

            // if arrays disagree proceed to next iter
            if sub_c != &s_vec[i+j] { 
                same_so_far = false;
                break;
            }
        }

        // if we finish sub-iter the same, we're done
        if same_so_far {
            return true;
        }
    }
    false
}

#[test]
fn test_string_has_substr() {
    assert!(string_has_substr("hello", "h"));
    assert!(string_has_substr("hello", "l"));
    assert!(string_has_substr("hello", "o"));
    assert!(string_has_substr("hello", "hel"));
    assert!(string_has_substr("hello", "llo"));
    assert!(string_has_substr("hello", "ell"));

    // edge
    assert!(!string_has_substr("", ""));
    assert!(string_has_substr("h", ""));
    
    // do not have
    assert!(!string_has_substr("hello", "b"));
    assert!(!string_has_substr("hello", "bye"));
    assert!(!string_has_substr("hello", "helloo"));
    assert!(!string_has_substr("hello", "lloo"));
}
