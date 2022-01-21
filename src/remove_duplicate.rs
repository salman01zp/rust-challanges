/// Remove duplicate from the sorted array
/*
    if array is sorted you can directly used vec.dedup()


    example : [0,0,1,1,1,2,2,3,3,4]
            since it is sorted array we can compare consecutive number

alternate approch
use vec::dedup method, it only works if vector is sorted in case of unsorted first sort and then used dedup

fn remove_duplicate(input: &mut Vec<i32>) -> i32 {
    input.sort();
    input.dedup();
}
*/

#![allow(unused_variables)]
fn remove_duplicates(input:&mut Vec<i32>) ->u32 {
    let mut last = input[0];
    let mut size = 1;
    for i in 1..input.len(){
        if input[i] != last {
            last = input[i];
            input[size] = input[i];
            size +=1;
        }
    }
    size as u32
}

    
fn main(){
    let mut input = vec![0,0,1,1,1,2,2,3,3];
    let size = remove_duplicates(&mut input);
    println!("{}",size);
    println!("{:?}",&input[..size as usize]);
}
    
