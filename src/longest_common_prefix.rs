struct Solution;

impl Solution {
    fn find_longest_common_prefix(input_str:Vec<String>) -> String {
        if input_str.is_empty() {
            return String::from("");
        }
        let mut prefix: Vec<char> = vec![];
        let ss: Vec<Vec<char>> = input_str.iter().map(|s| s.chars().collect()).collect();
        let n = ss.iter().map(|s| s.len()).min().unwrap();

        for i  in 0..n {
            let c = ss[0][i];
            if ss.iter().all(|s| s[i] == c) {
                prefix.push(c);
            } else {
                break;
            }
        }
        prefix.iter().collect()
    }
}
#[test]
fn simple_test_case() {
    let input:Vec<String> = ["flower","dsvdsfly","flight"].iter().map(|s| s.to_string()).collect();
    let output = Solution::find_longest_common_prefix(input);
    assert_eq!(input,output.to_string());
}