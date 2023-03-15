fn main() {
    println!("median-of-two-sorted-arrays");
}

trait Solution {
    fn solution(nums1: Vec<i32>, nums2: Vec<i32>) -> f64;
}

struct CheatingSolution;

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
            median = nums1_mut[num_len / 2] as f64;
        }

        median
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
}
