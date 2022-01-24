/*
nums = []  // non empty
find no which is not duplicated

example  :
    nums = [2,2,1]
    output = 1

Approach 1:
    XOR product will return unique number
*/


fn single_number(nums:Vec<i32>)->i32 {
    nums.iter().fold(|res,val| res ^ val);
}

#[test]
fn test_case1(){
    let nums = vec![1,2,2,3,3];
    let res = single_numbser(nums);
    assert_eq!(res,1)
}

