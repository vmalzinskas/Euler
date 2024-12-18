use crate::prob6::problem_6;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};
use std::ops::Div;

/// If a box contains twenty-one coloured discs, composed of fifteen blue discs and six red discs,
/// and two discs were taken at random, it can be seen that the probability of taking two blue discs,
/// The next such arrangement, for which there is exactly
/// chance of taking two blue discs at random, is a box containing eighty-five blue discs and thirty-five red discs.
/// By finding the first arrangement to contain over
/// discs in total, determine the number of blue discs that the box would contain.
pub fn problem_100_a(min_disks: usize) {
    //We can start our search at the target minimum disk number multiplied to square root of 0.5
    let min = (min_disks as f32 * std::f32::consts::FRAC_1_SQRT_2).floor() as usize;
    let b: Box = Box::check_range(min_disks, min, min*1000).unwrap();
    let (blue, red) =  Box::predict_next(b);

    println!("The next predicted box has min disk: {}", blue + red);
}

pub fn problem_100_b(min_disks: usize) {
    let (blue, total) = match Box::function_solve(min_disks){
        Ok(result) => result,
        Err(err_msg) => {
            println!{"Error {}", err_msg};
            return
        }
    };
    println!("This box contains blue: {}, and red: {} ", blue, total-blue.clone());
}
pub fn problem_100_c(blue: usize, red: usize) {
    let b: Box = Box::check_range(red, blue,blue*100).unwrap();

}
pub fn problem_100_d(blue: usize, red: usize) {
    //We can start our search at the target minimum disk number multiplied to square root of 0.5
    let b: Box = Box::check_range(red, blue, (blue+red)*1000).unwrap();
    let (blue, red) = b.get_box_tally();
    println!("predicted next box: ({}, {})", blue as f64 * 5.8284017630137, red as f64 * 5.8284017630137);
}


struct Box {
    blue: usize,
    red: usize,
}

impl Box {
    fn new() -> Self {
        Box {blue: 0, red: 0,}
    }
    fn new_filled(blue: usize, red: usize) -> Self {
        Box {blue: blue, red: red,}
    }
    fn add_red(&mut self) {self.red += 1 ;}
    fn add_blue(&mut self) {self.blue += 1 ;}

    fn get_box_tally(&self) -> (usize, usize) {
        (self.blue, self.red)
    }

    fn get_disk_n(&self) -> usize {
        self.blue + self.red
    }

    /// This checks if the probability of 2 blues is 0.5
    fn check_prob(&self) -> Result<bool, &'static str> {
        if self.blue >=2 {
            let (blue, _) = self.get_box_tally();
            let blue = BigInt::from(blue);
            let disk_total =  BigInt::from(self.get_disk_n());
            let numer =  blue.clone()*(blue.clone() - BigInt::one());
            let denom =  disk_total.clone()* (disk_total.clone()-BigInt::one());
            // Compare the probabilities without division
            let left_side = numer * BigInt::from(100); // Numerator scaled by 100
            let right_side = denom.clone() * BigInt::from(50); // 50% scaled by denom
            if left_side == right_side {
                // println!("success");
                Ok(true)
            }else {
                // println!(
                //     "blue: {}, red: {}, fail (numerator={}, denominator={}, prob_scaled={})",
                //     blue.clone(),
                //     disk_total.clone() - blue.clone(),
                //     left_side.clone(),
                //     denom.clone(),
                //     left_side.clone() / denom.clone(), // For debugging only
                // );
                return Ok(false);
            }
        } else {
            Err("Minimum number of blue disks are not present.")
        }
    }
    /// Both probability of the first pick must be between [0.5 and 1] * the disk number as
    /// these are the only numbers that could have a partner probability that multiplies to 0.5
    /// I can only use intuitive logic for this. It makes no sense that the first disk could ever have
    /// a probability of 1 as that would require the next disk to have a probability of 0.5. Which
    /// means we go from having zero red disks to having half of all disks being red. As the disk box
    /// gets larger and the disk contained approach infinity then the removal of the first disk makes
    /// less and less difference, ie the probabilities approach each other and are therefore approaching
    /// the square root of 0.5 which is equivalent to 0.707107. So if we quickly check close numbers
    /// then check exact numbers we might be able to do it.
    fn check_range(red: usize, min_blue: usize, max: usize) -> Result<Box, String>{
        let mut prob_box =  Box::new_filled(min_blue, red);
        //println!("blue {}, red {}", min_blue, red);
        if prob_box.balance(max) {
            let (blue, red) = prob_box.get_box_tally();
            println!("The first box found in the domain [{}, {}] contains {} blue, and {} red disks.", min_blue, max, blue, red);
            return Ok(Box::new_filled(blue, red))
        }else {
            println!("Could not find a box in this range that matches the criteria.");
            return Err("Could not find a box in this range that matches the criteria.".to_string())
        }
    }

    /// I want to balance the box as close to possible so the first blue is close to 0.707107
    fn balance(&mut self, max: usize) -> bool {
        loop {
            let (blue, red) = self.get_box_tally();
            //println!("blue {}, red {}, prob {}", blue, red, self.check_prob().unwrap());
            if blue + red >= max { return false; }
            if self.check_prob().unwrap() {
                return true;
            }else if self.check_ratio() <= std::f32::consts::FRAC_1_SQRT_2 {
                self.add_blue();
            }else{
                self.add_red(); //println!("added red\n");
            }

        }
    }

    /// check_ratio finds the ratio of the blue disks to total
    fn check_ratio(&self) -> f32 {
        let (blue, red) = self.get_box_tally();
        blue as f32 / self.get_disk_n() as f32
    }
    /// Get the ratio of a single blue disk to the total disks
    /// This will be used for balancing the box. It will give finite values to the ranges used in
    /// determinining the direction needed to balance.
    fn get_single_disk_ratio(&self) -> f32 {
        1.0 / self.get_disk_n() as f32
    }
    /// The growth of the relationship between blue and red seems linear. We can use the x distance to
    /// estimate the next box .
    /// using the linear regression formula y = 0.4142x - 0.2145
    /// This has been successful up until the 14th box. But not after that.
    fn predict_next(b: Box) -> (usize, usize) {
        let (blue, red) =  b.get_box_tally();
        let current_x: f64 = blue as f64;
        let next_x = current_x * 1.998;
        let next_y:f64 =  0.4142 * next_x - 0.2145;
        (next_x.floor() as usize, next_y.floor() as usize)
    }

    /// Solving using a function for the total number of disks
    fn function_solve(min: usize) -> Result<(BigInt, BigInt), String> {
        let max = BigInt::from(min) * BigInt::from(1000);
        let mut x = BigInt::from(min);

        while x <= max {
            // Calculate the discriminant: 8 * x^2 - 8 * x + 1
            let discriminant:BigInt = &x * &x * 8 - &x * 8 + BigInt::one();

            // Check if the discriminant is a perfect square
            let sqrt_discriminant = discriminant.sqrt(); // Using num-integer's sqrt method or custom logic
            if &sqrt_discriminant * &sqrt_discriminant == discriminant {
                // Calculate y
                let y = (&sqrt_discriminant + BigInt::one()).div(2);
                if (&sqrt_discriminant + BigInt::one()) % 2 == BigInt::zero() {
                    return Ok((x.clone(), y));
                }
            }

            x += BigInt::one(); // Increment x
        }

        Err("No result".to_string())
    }
}

#[test]
fn test_function_solve(){
    let (x,y) = Box::function_solve(4).unwrap();
    assert_eq!((x,y), (BigInt::from(15),BigInt::from(21)))
}


#[test]
fn test_box_tally(){
    let mut prob_box = Box::new();
    prob_box.add_blue();
    prob_box.add_blue();
    prob_box.add_red();
    assert_eq!(prob_box.get_disk_n(), 3);
    assert_eq!(prob_box.get_box_tally(), (2, 1));
}

#[test]
fn test_prob_two_blue() {
    let prob_box = Box::new_filled(15, 6);
    assert!(prob_box.check_prob().unwrap());
    let prob_box = Box::new_filled(85, 35);
    assert!(prob_box.check_prob().unwrap());
    let prob_box = Box::new_filled(493, 204);
    assert!(prob_box.check_prob().unwrap());
    let prob_box = Box::new_filled(2871, 1189);
    assert!(prob_box.check_prob().unwrap());
    let prob_box = Box::new_filled(16731, 6930);
    assert!(prob_box.check_prob().unwrap());
    let prob_box = Box::new_filled(97513, 40391);
    assert!(prob_box.check_prob().unwrap());
    let prob_box = Box::new_filled(568345, 235416);
    assert!(prob_box.check_prob().unwrap());
    let prob_box = Box::new_filled(3312543, 1372099);
    assert!(prob_box.check_prob().unwrap());
    let prob_box = Box::new_filled(3312543, 1372099);
    assert!(prob_box.check_prob().unwrap());
    let prob_box = Box::new_filled(1000000001415, 414213562959);
    assert!(!prob_box.check_prob().unwrap());
}

#[test]
fn test_check_ratio(){
    let prob_box = Box::new_filled(85, 35);
    assert_eq!(prob_box.check_ratio(), (85.0/120.0) as f32);
    let prob_box = Box::new_filled(2, 0);
    assert_eq!(prob_box.check_ratio(), (2.0/2.0) as f32);
}

#[test]
fn test_single_disk_ratio(){
    let prob_box = Box::new_filled(85, 35);
    assert_eq!(prob_box.get_single_disk_ratio(), (1.0/120.0) as f32)
}

#[test]
fn test_balance(){
    Box::check_range(200,100, 500);
    // unfinished test
}

#[test]
fn test_predict(){
    let b = Box::new_filled(353579,146457);
    let (blue, red) = Box::predict_next(b);
    assert_eq!((blue, red), (706450, 292611));
}