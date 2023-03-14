use std::cmp::max;
use std::collections::HashSet;

fn main() {
    println!("longest-substring-without-repeating-characters");
}

trait Solution {
    fn solution(s: String) -> i32;
}

struct SlidingWindow;

impl Solution for SlidingWindow {
    fn solution(s: String) -> i32 {
        let str_len = s.len();
        let mut start = 0;
        let mut end = 0;
        let mut max_len = 0;
        let mut repeated_chars = HashSet::new();

        // convert to bytes, assuming its only ASCII
        let s_bytes = s.as_bytes();

        while start < str_len && end < str_len {
            if !repeated_chars.contains(&s_bytes[end]) {
                repeated_chars.insert(s_bytes[end]);
                end +=1;
                max_len = max(max_len, end - start);
            } else {
                repeated_chars.remove(&s_bytes[start]);
                start+=1;
            }
        }
        max_len as i32
    }
}
