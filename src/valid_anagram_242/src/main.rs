// 242. Valid Anagram
// Given two strings s and t, return true if t is an anagram of s, and false otherwise.

// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

// Example 1:

// Input: s = "anagram", t = "nagaram"
// Output: true
// Example 2:

// Input: s = "rat", t = "car"
// Output: false

// Constraints:

// 1 <= s.length, t.length <= 5 * 104
// s and t consist of lowercase English letters.

use std::collections::HashMap;
fn main() {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            if let Some(x) = map.get_mut(&c) {
                *x -= 1;
                if *x == 0 {
                    map.remove(&c);
                }
            } else {
                return false;
            }
        }
        map.is_empty()
    }
}
