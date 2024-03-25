/**
**/
struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let mut prefix_len = strs[0].len();

        for s in strs.iter().skip(1) {
            let mut i = 0;
            while i < prefix_len && i < s.len() && strs[0].as_bytes()[i] == s.as_bytes()[i] {
                i += 1;
            }
            prefix_len = i; // Update the length
        }

        strs[0][..prefix_len].to_string()
    }
}
#[cfg(test)]
