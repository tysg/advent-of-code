use std::collections::VecDeque;

pub fn solve(input: &str) {
    let mut arr = input.lines().map(|l| l.parse::<u32>().unwrap()).collect::<Vec<_>>();
    arr.sort();
    arr.push(arr[arr.len()-1] + 3);
    arr.push(0);
    arr.sort();



    let mut dp: Vec<u128> = vec![0; arr.len()];
    dp[0] = 1;
    let l = arr.len();

    for i in 0..l {
        if i + 1 < l && (arr[i+1]-arr[i] <= 3){
            dp[i+1] += dp[i];
        }
        if i +2 < l &&(arr[i+2]-arr[i] <= 3){
            dp[i+2] += dp[i];
        }

        if i+3 < l&&(arr[i+3]-arr[i] <= 3){
            dp[i+3] += dp[i];
        }
    }

    println!("{}", dp[l-1]);













    let mut one_diff = 0;
    let mut three_diff = 0;



    
    for i in 1..arr.len() {
        match arr[i] - arr[i-1] {
            1 => one_diff += 1,
            3 => three_diff += 1,
            _ => ()
        }
    }

    println!("{}", one_diff * (three_diff));

}
