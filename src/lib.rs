pub mod prob6;
pub mod prob100;
///// Problem 1 /////////////
fn get_multiples(of : i32, max : i32) -> Vec<i32> {
    let mut multiples = Vec::new();
    for i in of..max {
        if i % of == 0 {
            multiples.push(i);
        }
    }
    multiples.sort();
    multiples
}
fn get_all_multiples(list_of: Vec<i32>, max: i32) -> Vec<i32> {
    let mut multiples: Vec<i32> = Vec::new();
    for i in list_of {
        multiples.extend(get_multiples(i, max));
    }
    multiples.sort();
    multiples.dedup();
    multiples
}

fn sum_all_multiples(multiples: Vec<i32>) -> i32 {
    multiples.iter().sum()
}

pub fn problem_1(list_of: Vec<i32>, max: i32) {
    let sum = sum_all_multiples(get_all_multiples(list_of.clone(), max));
    println!("The sum of all the mulitples of {:?} below {} is {}", list_of, max, sum);
}


//////// Problem 4 ////////////

fn is_palindrome(number :&i32) -> bool {
    *number == reverse_number(number)
}

fn reverse_number(number :&i32) -> i32 {
    let mut reversed = 0;
    let mut n = *number;
    while n > 0 {
        reversed = n % 10 + (reversed * 10);
        n /= 10;
    }
    reversed
}
fn get_all_numbers_of_len_n(n: u32) -> Result<Vec<i32>, String> {
    if n > 0 {
        let start :i32 = 10_i32.pow(n - 1);
        let end :i32 = 10_i32.pow(n);
        let numVec: Vec<i32> = (start..end).rev().collect();
        Ok(numVec)
    } else {
        Err("n must be larger than 0".to_string())
    }
}
pub fn problem_4(n: u32) {
    let result =  get_all_numbers_of_len_n(n);
    let mut num_list :Vec<i32> = Vec::new();
    match result {
        Ok(value) => num_list = value,
        Err(e) => println!("Error: {}", e),
    }
    let mut multi_list: Vec<i32> = num_list
                        .iter()
                        .flat_map(|&x| num_list.iter().map(move |&y| x * y))
                        .collect();
    multi_list.sort();
    for num in multi_list.iter().rev(){
        if is_palindrome(num){
            println!("The largest palindrome is {}", num);
            break;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    //////////// Problem 1 ////////////
    #[test]
    fn test_get_multiples() {
        let expected = vec!(3, 6, 9);
        let actual = get_multiples(3, 10);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_multiples_1() {
        let expected = vec!(3, 6, 9);
        let actual = get_multiples(4, 10);
        assert_ne!(expected, actual);
    }

    #[test]
    fn test_get_all_multiples() {
        let expected = vec!(3, 5, 6, 9);
        let actual = get_all_multiples(vec!(3,5), 10);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sum_all_multiples() {
        let actual = get_all_multiples(vec!(3,5), 10);
        assert_eq!(23, sum_all_multiples(actual));
    }

    #[test]
    fn test_removal_duplicates() {
        let expected = vec!(3, 6, 9);
        let actual = get_all_multiples(vec!(3,6), 10);
        assert_eq!(expected, actual);
    }

    /////////// Problem 4 ///////////
    #[test]
    fn test_reversed() {
        let actual : i32 = 985454589;
        let expected: i32 = reverse_number(&985454589);
        assert_eq!(expected, actual);

        let actual : i32 = 9019100;
        let expected: i32 = reverse_number(&9019100);
        assert_ne!(expected, actual);
    }
    #[test]
    fn test_is_palidrome() {
        let number: i32 = 985454589;
        assert!(is_palindrome(&number));
        let number: i32 = 111111;
        assert!(is_palindrome(&number));
        let number: i32 = 1222223;
        assert!(!is_palindrome(&number));
    }
    #[test]
    fn test_get_all_numbers_of_len_n(){
        let singles_expected = vec![9,8,7,6,5,4,3,2,1];
        let singles_actual = get_all_numbers_of_len_n(1);
        assert_eq!(Ok(singles_expected), singles_actual);
        let doubles_expected = vec![99,98,97,96,95,94,93,92,91,90,
                                            89,88,87,86,85,84,83,82,81,80,
                                            79,78,77,76,75,74,73,72,71,70,
                                            69,68,67,66,65,64,63,62,61,60,
                                            59,58,57,56,55,54,53,52,51,50,
                                            49,48,47,46,45,44,43,42,41,40,
                                            39,38,37,36,35,34,33,32,31,30,
                                            29,28,27,26,25,24,23,22,21,20,
                                            19,18,17,16,15,14,13,12,11,10];
        let doubles_actual = get_all_numbers_of_len_n(2);
        assert_eq!(Ok(doubles_expected), doubles_actual);
    }
}
