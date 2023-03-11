use std::collections::HashMap;

fn main() {
    println!("Two Sum soluitions");
}

trait TwoSum {
    fn solution(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

struct BruteForce;
struct HashTable;

impl TwoSum for BruteForce {
    fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            for j in i + 1..n {
                if (nums[i] + nums[j]) == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
        return vec![]
    }
}

impl TwoSum for HashTable {
    fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_table: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            let complement = target - num;
            if let Some(val) = hash_table.get(&complement) {
                return vec![*val as i32, i as i32]
            }
            hash_table.insert(num, i);
        }
        return vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;
   
    #[test]
    fn brute_force_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(BruteForce::solution(nums, target), vec![0, 1])
    }


    #[test]
    fn brute_force_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(BruteForce::solution(nums, target), vec![1, 2])
    }


    #[test]
    fn brute_force_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(HashTable::solution(nums, target), vec![0, 1])
    }

    #[test]
    fn hash_map_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(HashTable::solution(nums, target), vec![0, 1])
    }


    #[test]
    fn hash_map_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(HashTable::solution(nums, target), vec![1, 2])
    }


    #[test]
    fn hash_map_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(HashTable::solution(nums, target), vec![0, 1])
    }
}
