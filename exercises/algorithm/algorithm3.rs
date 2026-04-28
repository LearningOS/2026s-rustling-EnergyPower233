/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::mem::swap;

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let mut lt = 0;
    let mut gt = array.len() - 1;
    let mut curr = 1;

    while curr <= gt {
        if array[curr] < array[lt] {
            array.swap(curr, lt);
            lt += 1;
            curr += 1;
        } else if array[curr] > array[lt] {
            array.swap(curr, gt);
            gt -= 1;
        } else {
            curr += 1;
        }
    }
    sort(&mut array[0..lt]);
    sort(&mut array[gt + 1..]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
