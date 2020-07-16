fn count(nums: Vec<i32>) -> i32 {
    let depth = nums.len();
    if depth < 1 {
        return 0;
    }
    if depth == 1 {
        return nums[0];
    };
    let len = nums[0];
    let mut v = vec![1; len as usize];
    for i in 1..depth {
        let len = nums[i - 1] as usize;
        let len2 = nums[i] as usize;
        if (len as i32 - len2 as i32).abs() > 1 || len < 1 || len2 < 1 {
            unreachable!()
        };
        let mut v2 = vec![0; len2 as usize];
        if len2 > len {
            v2[0] = v[0];
            v2[len2 - 1] = v[len - 1];
            for i in 1..len {
                v2[i] = v[i - 1] + v[i];
            }
        } else if len2 < len {
            for i in 0..len2 {
                v2[i] = v[i] + v[i + 1]
            }
        } else {
            for i in 0..len {
                v2[i] = v[i]
            }
        }
        v = v2.to_vec()
    }
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(count(vec![1, 2, 3, 4, 3, 2, 3, 2]), 60)
    }

    #[test]
    fn test_count_02() {
        assert_eq!(count(vec![1, 2, 3]), 4)
    }

    #[test]
    fn test_count_03() {
        assert_eq!(count(vec![1, 2, 3, 2, 1]), 6)
    }

    #[test]
    fn test_count_04() {
        assert_eq!(count(vec![1, 2, 3, 3, 3]), 4)
    }

    #[test]
    #[should_panic]
    fn test_count_05() {
        assert_eq!(count(vec![1, 2, 0]), 0)
    }
}
