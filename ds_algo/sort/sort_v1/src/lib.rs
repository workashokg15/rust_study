use std::fmt::Debug;
use std::time::Instant;

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


pub fn merge_sort_pd<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T>
{
    //println!("{:?}", v);
    if v.len() <= 1 {
        return v;
    }
    let b = v.split_off(v.len()/2);
    let a = merge_sort_pd(v);
    let b = merge_sort_pd(b);
    merge_pd(a, b)
}

fn merge_pd<T: PartialOrd + Debug>(mut a: Vec<T>, mut b: Vec<T>)->Vec<T>
{
    let mut result = Vec::with_capacity(a.len() + b.len());
    while !a.is_empty() && !b.is_empty() {
        if a[0] <= b[0] {
            result.push(a.remove(0));
        } else {
            result.push(b.remove(0));
        }
    }    
    result.extend(a);
    result.extend(b);
    //println!("{:?}", result);
    result
}

pub fn merge_sort(a: &mut Vec<i32>, left: usize, right: usize)
{
    if right > left {
        let mid= (left + right)/2;
        merge_sort(a, left, mid);
        merge_sort(a, mid+1, right);
        merge(a, left, mid, right);
    }
}

fn merge(v: &mut Vec<i32>, left: usize, mid: usize, right: usize)
{
    let left_vec = v[left..=mid].to_vec();
    let right_vec = v[mid+1..=right].to_vec();

    let mut left_itr = left_vec.iter().peekable();
    let mut right_itr = right_vec.iter().peekable();

    for i in left..=right {
        match(left_itr.peek(), right_itr.peek()) {
            (None, Some(_)) => v[i] = *right_itr.next().unwrap(),
            (Some(_), None) => v[i] = *left_itr.next().unwrap(),
            (Some(l), Some(r)) if l > r => v[i] = *right_itr.next().unwrap(),
            (Some(l), Some(r)) if l < r => v[i] = *left_itr.next().unwrap(),
            (Some(_), Some(_)) => v[i] = *left_itr.next().unwrap(),
            _ => break
        }
    }
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
    #[test]
    fn test_merge_sort(){
        let mut v  = vec![4,6,1,8,11,13,3];
        let length = v.len();
        merge_sort(&mut v, 0, length-1);
        assert_eq!(v, [1,3,4,6,8,11,13])
    }
    #[test]
    fn test_merge_sort_pd(){
        let v  = vec![4.2, 6.0, 1.0, 8.0, 11.0, 13.0, 3.0]; // Use floating-point numbers
        let start_time = Instant::now();
        let sorted_v = merge_sort_pd(v);
        let elapsed_time = start_time.elapsed();
        let milliseconds = (elapsed_time.as_secs() as f64 * 1000.0) + (elapsed_time.subsec_nanos() as f64 / 1_000_000.0);
        println!("\nOptimal sort elapsed time: {} ms", milliseconds);
        assert_eq!(sorted_v, [1.0, 3.0, 4.2, 6.0, 8.0, 11.0, 13.0])
    }
}
