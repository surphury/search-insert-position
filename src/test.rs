#[cfg(test)]
use crate::search_insert;

#[test]
fn test_1() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    let result = search_insert(nums, target);
    assert_eq!(result, 2);
}

#[test]
fn test_2() {
    let nums = vec![1, 3, 5, 6];
    let target = 2;
    let result = search_insert(nums, target);
    assert_eq!(result, 1);
}

#[test]
fn test_3() {
    let nums = vec![1, 3, 5, 6];
    let target = 7;
    let result = search_insert(nums, target);
    assert_eq!(result, 4);
}
