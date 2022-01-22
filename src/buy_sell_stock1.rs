#![allow(dead_code)]
/// Best time to Buy and Sell Stock
/* 
    Example:
        prices = [7,1,5,3,6,4]
        output = 5

    BUY : day2
    kadane Algorithm
    global max_profit = 0;
    current_profit = 0

    Brute Force method = O(no_days*no_days)
    keep track of profit  = O(n)


*/


fn find_max_profit2(prices:Vec<i32>) -> i32 {
    // Brute force approach
    let mut max_profit = 0;
    let mut current_profit = 0;
    let n = prices.len();
    for i in 0..n-1 {
        for j in i+1..n {
            current_profit = prices[j] - prices[i];
            if current_profit > max_profit {
                max_profit = current_profit;
            }

        }

    }
    max_profit
}


fn find_max_profit(prices:Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min_price  = prices[0];
    let n = prices.len();
    for i in 0..n {
        if prices[i] < min_price {
            min_price = prices[i];
        } else {
            if (prices[i]-min_price) > max_profit {
                max_profit = prices[i] - min_price;
            }
        }
    }

    max_profit as i32
    
}


#[test]
fn test_case1(){
    let prices = vec![7,1,5,3,6,4];
    let profit = find_max_profit(prices);
    assert_eq!(profit,5)
}

#[test]
fn test_case2(){
    let prices = vec![9,3,8,3,6,4];
    let profit = find_max_profit(prices);
    assert_eq!(profit,5)
}

#[test]
fn test_case3(){
    let prices = vec![3,1,9,3,4,4];
    let profit = find_max_profit(prices);
    assert_eq!(profit,8)
}