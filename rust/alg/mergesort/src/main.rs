fn merge(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut v = Vec::new();
    let mut v1_iter = v1.iter();
    let mut v1_val = v1_iter.next();
    let mut v2_iter = v2.iter();
    let mut v2_val = v2_iter.next();
    loop {
        if v1_val.is_none() {
            if v2_val.is_none() {
                break;
            }
            v.push(v2_val.unwrap().clone());
            v2_val = v2_iter.next();
        } else {
            if v2_val.is_none() {
                v.push(v1_val.unwrap().clone());
                v1_val = v1_iter.next();
            } else {
                if v1_val < v2_val {
                    v.push(v1_val.unwrap().clone());
                    v1_val = v1_iter.next();
                } else {
                    v.push(v2_val.unwrap().clone());
                    v2_val = v2_iter.next();
                }
            }
        }
    }
    return v;
}

pub fn mergesort(v: Vec<i32>) -> Vec<i32> {
    if v.len() <= 1 {
        return v;
    }
    let v1 = mergesort((&v[0..v.len() / 2]).to_vec());
    let v2 = mergesort((&v[v.len() / 2..]).to_vec());
    return merge(v1, v2);
}

// end is exclusive
fn shift_r(v: &mut Vec<i32>, start: usize, end: usize) {
    assert!(
        start < v.len(),
        "start value too large for vec {} >= {}",
        start,
        v.len()
    );
    assert!(end > 0);
    assert!(end < v.len(), "end value too large for vec");

    let mut i = end - 1;
    while i >= start {
        v[i + 1] = v[i];
        if i == start {
            break;
        }
        i -= 1;
    }
}

#[test]
fn test_shift_r() {
    let v2 = &mut vec![1, 2];
    shift_r(v2, 0, 1); // only move the first
    assert!(v2 == &vec![1, 1]);

    let v3 = &mut vec![1, 2, 3];
    shift_r(v3, 0, 2); // move first two
    assert_eq!(v3, &vec![1, 1, 2]);
}

// definitely not idiomatic rust
fn merge_inplace(v: &mut Vec<i32>, start: usize, boundary: usize, end: usize) {
    // println!("merge_inplace: {:?} {} {} {} ", v, start, boundary, end);
    let mut v1 = start;
    let mut v2 = boundary; // start of v2

    loop {
        if v1 >= v2 {
            if v2 >= end {
                // println!("merge_inplace: done w both lists!");
                break;
            }
            // println!("merge_inplace: done with only first");
            let temp = v[v2];
            shift_r(v, v1, v2);
            v[v1] = temp;
            v1 += 1;
            v2 += 1;
        } else {
            if v2 >= end {
                // done with v2, so nothing to cause v1 shift
                // println!("merge_inplace: done w only second!");
                break;
            } else {
                if v[v1] < v[v2] {
                    // println!("v1 < v2: {} < {}", v[v1], v[v2]);
                    // v1 already sorted, just move the ptr
                    v1 += 1;
                } else {
                    // println!("v1 >= v2: {} >= {}", v[v1], v[v2]);
                    let temp = v[v2];
                    shift_r(v, v1, v2);
                    v[v1] = temp;
                    v1 += 1;
                    v2 += 1;
                }
            }
        }
        // println!("looping: {:?}", v);
    }
}

fn mergesort_inplace_inner(v: &mut Vec<i32>, start: usize, end: usize) {
    // println!("mergesort_inplace_inner: {:?} {} {} ", v, start, end);
    if end < start {
        assert!(false, "end should never be less than start");
        return;
    }
    if end - start <= 1 {
        // println!("end - start <= 1, end - start: {}", end - start);
        return;
    }
    mergesort_inplace_inner(v, start, (end - start) / 2 + start);
    mergesort_inplace_inner(v, (end - start) / 2 + start, end);
    merge_inplace(v, start, (end - start) / 2 + start, end);
    // println!("{:?}", v);
}

pub fn mergesort_inplace(v: &mut Vec<i32>) {
    if v.len() <= 1 {
        return;
    };
    mergesort_inplace_inner(v, 0, v.len());
}

#[test]
fn test_mergesort_basic() {
    assert!(mergesort(vec![]) == vec![]); // empty lists
    assert!(mergesort(vec![1]) == vec![1]); // single value
    assert!(mergesort(vec![1, 6]) == vec![1, 6]); // unique,unsorted
    assert!(mergesort(vec![1, 6, 4, 9, 3, 0]) == vec![0, 1, 3, 4, 6, 9]); // unique,unsorted
    assert!(mergesort(vec![1, 0, 0, 1, 2, 0]) == vec![0, 0, 0, 1, 1, 2]); // dups
}

#[test]
fn test_mergesort_inplace_basic() {
    let v0 = &mut vec![];
    mergesort_inplace(v0);
    assert_eq!(v0, &mut vec![]); // empty list

    let v1 = &mut vec![1];
    mergesort_inplace(v1);
    assert_eq!(v1, &mut vec![1]); // single value

    let v2 = &mut vec![1, 6, 4, 9, 3, 0];
    mergesort_inplace(v2);
    assert_eq!(v2, &mut vec![0, 1, 3, 4, 6, 9]); // unique list

    let v3 = &mut vec![1, 0, 0, 1, 2, 0];
    mergesort_inplace(v3);
    assert_eq!(v3, &mut vec![0, 0, 0, 1, 1, 2]); // dup list

    let v4 = &mut vec![1, 6, 4, 9, 3, 0, -1, 100, 1000, 7, 2, 10, 5];
    mergesort_inplace(v4);
    assert_eq!(v4, &mut vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 100, 1000]); // unique list
}

fn main() {
    // do nothing
}
