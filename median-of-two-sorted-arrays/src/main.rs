use std::cmp::max;
use std::cmp::min;

fn main() {
    println!("median-of-two-sorted-arrays");
}

trait Solution {
    fn solution(nums1: Vec<i32>, nums2: Vec<i32>) -> f64;
}

struct CheatingSolution;
struct ComplexityONPlusM;
struct BinarySearch;

impl Solution for CheatingSolution {
    fn solution(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // make vectors mutable so we could merge and sort them
        let mut nums1_mut = nums1;
        let mut nums2_mut = nums2;

        nums1_mut.append(&mut nums2_mut);
        nums1_mut.sort();

        let num_len = nums1_mut.len();
        let median: f64;
        let quotient = num_len / 2;

        if num_len % 2 == 0 {
            median = (nums1_mut[quotient - 1] as f64 + nums1_mut[quotient] as f64) / 2.0;
        } else {
            median = nums1_mut[quotient] as f64;
        }
        median
    }
}

impl Solution for ComplexityONPlusM {
    fn solution(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let n = n1 + n2;
        let mut new_nums = vec![0; n];

        let (mut i, mut j, mut k) = (0, 0, 0);

        while i <= n1 && j <= n2 {
            if i == n1 {
                while j < n2 {
                    new_nums[k] = nums2[j];
                    k += 1;
                    j += 1;
                }
                break;
            } else if j == n2 {
                while i < n1 {
                    new_nums[k] = nums1[i];
                    k += 1;
                    i += 1;
                }
                break;
            }
            if nums1[i] <= nums2[j] {
                new_nums[k] = nums1[i];
                k += 1;
                i += 1;
            } else {
                new_nums[k] = nums2[j];
                k += 1;
                j += 1;
            }
        }

        let quotient = n / 2;
        if n % 2 == 0 {
            return (new_nums[quotient - 1] as f64 + new_nums[quotient] as f64) / 2.0;
        }
        new_nums[quotient] as f64
    }
}

impl Solution for BinarySearch {
    fn solution(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let a;
        let b;

        if nums1.len() < nums2.len() {
            a = nums1;
            b = nums2;
        } else {
            a = nums2;
            b = nums1;
        }

        let n1 = a.len();
        let n2 = b.len();
        let n = n1 + n2;
        let half = n / 2;
        let (mut l, mut r) = (0, a.len() - 1);

        loop {
            let i = (l + r) / 2;
            let j = half - i - 2;

            let a_left = a[i];
            let a_right = a[i + 1];
            let b_left = b[j];
            let b_right = b[j + 1];

            if a_right <= b_left && b_right <= b_left {
                if n % 2 == 0 {
                    return max(a_left, b_left) as f64 + min(a_right, b_right) as f64 / 2.0;
                }
                return min(a_right, b_right) as f64;
            } else if a_left > b_right {
                r += 1;
            } else {
                l -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_1() {
        let in1 = vec![1, 3];
        let in2 = vec![2];
        assert_eq!(CheatingSolution::solution(in1, in2), 2 as f64);
    }

    #[test]
    fn solution_2() {
        let in1 = vec![1, 2];
        let in2 = vec![3, 4];
        assert_eq!(CheatingSolution::solution(in1, in2), 2.5 as f64);
    }

    #[test]
    fn solution_3() {
        let in1 = vec![1, 3];
        let in2 = vec![2];
        assert_eq!(ComplexityONPlusM::solution(in1, in2), 2 as f64);
    }

    #[test]
    fn solution_4() {
        let in1 = vec![1, 2];
        let in2 = vec![3, 4];
        assert_eq!(ComplexityONPlusM::solution(in1, in2), 2.5 as f64);
    }
}
