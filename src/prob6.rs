
pub fn problem_6(n:u32) -> String {
    println!("The difference is: {}", square_sums(n)-sum_squares(n));
    format!("The difference is: {}", square_sums(n)-sum_squares(n))
}

fn sum_squares(n :u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 1..= n {
        sum += i*i;
    }
    sum
}

fn square_sums(n: u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 1..=n {
        sum += i;
    }
    sum * sum
}


#[test]
fn test_sum_squares(){
    let actual: u32 = sum_squares(10);
    let expected: u32= 385;
    assert_eq!(expected, actual);
}

#[test]
fn test_square_sums(){
    let actual: u32 = square_sums(10);
    let expected: u32 = 3025;
    assert_eq!(expected, actual);
}

#[test]
fn test_problem_6(){
    assert_eq!(problem_6(10), "The difference is: 2640".to_string());
}