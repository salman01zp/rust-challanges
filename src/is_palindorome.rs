/// For given string check if its valid plaindrome
//  Conditions 1: consider ornly alphanumeric values
//  Constratints: 's' consists only of printable Ascii characters

#![allow(dead_code)]

fn is_palindrome(s:String) ->bool {
    let s:Vec<char> = s.chars()
    .filter(|c| c.is_ascii_alphanumeric())
    .map(|c| c.to_ascii_lowercase())
    .collect();

    let a: String = s.iter().collect();
    let b: String = s.iter().rev().collect();

    a == b

}


#[test]
fn test_case1(){
    let s = "A man, a plan, a canal: Panama".to_string();
    assert_eq!(is_palindrome(s),true);
}

#[test]
fn test_case2(){
    let s = "A man".to_string();
    assert_eq!(is_palindrome(s),false);
}
#[test]
fn test_case3(){
    let s = "nitin".to_string();
    assert_eq!(is_palindrome(s),true);
}
