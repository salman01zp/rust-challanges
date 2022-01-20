// Given string containing characters determine if input has
// valid parantheses
// 
//  input: string
//   for each char if its open add to stack else pop from stack
//   
//  return bool

#![allow(dead_code)]
struct Solution;

impl Solution {
    fn is_valid(input:String) -> bool {
        let mut stack: Vec<char> = vec![];
        for c in input.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' =>  match stack.pop() {
                    Some(t) => {
                        if !(
                            (t == '{' && c == '}') ||
                            (t == '(' && c == ')') ||
                            (t == '[' && c == ']') 
                            ) {

                                return false;
                            }
                    }
                    None => {
                        return false;
                    }
                },
                _ => {}
            }
        }
        stack.is_empty()
    }
}

#[test]

fn simple_test_case() {
    let input = "{{()}}".to_string();
    let output = Solution::is_valid(input);
    assert_eq!(true,output);
    
}