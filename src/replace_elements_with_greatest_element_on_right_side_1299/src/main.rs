fn main() {
    println!("Hello, world!");
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut last_elem = -1;
        let mut result = arr.clone();
        for i in (0..result.len()).rev() {
            let temp = result[i];
            result[i] = last_elem;
            last_elem = std::cmp::max(last_elem, temp);
        }
        result[arr.len()] = -1;
        result
    }
}
