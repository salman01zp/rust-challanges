/*
    given integer array find subarray which has largest sum and return its sum
    input: Vec
    output: i32
    
    example:
    nums = [-2,1,-3,4,-1]
    output = 2

    example 2
    nums [1]
    output: 1

    example 3
    nums [0]
    output: 0

    Approach: 1 Divide and concur
    

    Approach 2 : Kadane's method
            famous for sub array sum

        Create a variable to store global maximum 
        create variable to store sum. Initialize with zero




*/

use std::cmp::max;
fn max_sub_array(nums: Vec<i32>) ->i32 {
    let mut max_sum = nums[0];
    let mut current_max = nums[0];
    let n = nums.len();
    for i in 1..n {
        current_max = max(nums[i],current_max + nums[i]);
        max_sum = max(current_max,max_sum)
        
    }
    max_sum

}

#[test]
fn test_case1(){
    let nums = vec![-2, 1, -3, 4, -1, 2, 2, -5, 4];
    let  max_sum = max_sub_array(nums);
    assert_eq(max_sum,7);
}

#[test]

fn test_case2(){
let nums = vec![1];
let  max_sum = max_sub_array(nums);
assert_eq(max_sum,1);
}