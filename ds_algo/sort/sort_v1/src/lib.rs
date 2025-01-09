use std::fmt::Debug;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

//O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        println!("{:?}", v);
        let mut sorted = true;
        for i in 0..(v.len()-1) - p {
            if v[i] > v[i+1] {
                v.swap(i, i+1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

//O(n * ln(n))
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    //sort the first half
    //sort the second half
    //merge both the sorted halfs together
    if v.len() <= 1 {
        return v;
    }
    let b = v.split_off(v.len()/2);
    let a = merge_sort(v);
    b = merge_sort(b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_bubble_sort() {
        let mut v=vec![4,6,1,8,11,13,3];
        bubble_sort(&mut v);
        assert_eq!(v, [1,3,4,6,8,11,13])
    }
}
