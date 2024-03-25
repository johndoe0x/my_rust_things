
/**
 * Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".

 

Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"
Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
 

Constraints:

1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] consists of only lowercase English letters.
**/ 

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let mut prefix = &strs[0];
        for character in strs.iter().skip(1) {
            let mut idx =0;
            while i < prefix.len() && idx < character.len() && prefix.chars().nth(idx) == s.chars().nth(idx) {
                idx += 1;
            }
            preix = & prefix[..i];
        }
        prefix.to_string() 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(Solution::longest_common_prefix(vec!["flower", "flow", "flight"]), "fl");
        assert_eq!(Solution::longest_common_prefix(vec!["dog", "racecar", "car"]), "");
    }
}

