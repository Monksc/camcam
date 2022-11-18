#![allow(dead_code)]
fn seen_before_helper(min: usize, max: usize, is_before: impl Fn(usize) -> bool) -> usize {
    let mut min = min;
    let mut max = max;
    while min < max {
        let mid = (min+max) / 2;
        if is_before(mid) {
            min = mid+1;
        } else {
            max = mid;
        }
    }

    return min;
}

pub fn seen_before<T : PartialOrd>(arr: &Vec<T>, x: T) -> usize {
    if arr.len() == 0 {
        return 0;
    }
    if arr[arr.len()-1] < x {
        return arr.len();
    }
    seen_before_helper(0, arr.len()-1, |index| -> bool { arr[index] < x })
}

pub fn seen_before_or_equal<T : PartialOrd>(arr: &Vec<T>, x: T) -> usize {
    if arr.len() == 0 {
        return 0;
    }
    if arr[arr.len()-1] < x {
        return arr.len();
    }
    seen_before_helper(0, arr.len()-1, |index| -> bool { arr[index] <= x })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_seen_before_zero_len() {
        assert_eq!(seen_before(&Vec::<f64>::new(), 0.0), 0);
    }

    #[test]
    pub fn test_seen_before() {
        let v = vec![
            4.0, 4.5, 10.0, 14.0, 14.0, 15.0, 16.3
        ];

        assert_eq!(seen_before(&v, 3.0), 0);
        assert_eq!(seen_before(&v, 4.0), 0);
        assert_eq!(seen_before(&v, 4.1), 1);
        assert_eq!(seen_before(&v, 4.6), 2);
        assert_eq!(seen_before(&v, 10.0), 2);
    }
}
