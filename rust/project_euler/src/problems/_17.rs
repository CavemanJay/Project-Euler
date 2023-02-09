use std::collections::HashMap;

use crate::utils::math::{self, DefaultMathType};
use lazy_static::lazy_static;

type Output = usize;
type Input = usize;

// const ones: HashMap<i32, &str> = HashMap::from_iter([
//     (1, "one"),
//     (2, "two"),
//     (3, "three"),
//     (4, "four"),
//     (5, "five"),
//     (6, "six"),
//     (7, "seven"),
//     (8, "eight"),
//     (9, "nine"),
// ]);

lazy_static! {
    static ref ONES: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "one");
        m.insert(2, "two");
        m.insert(3, "three");
        m.insert(4, "four");
        m.insert(5, "five");
        m.insert(6, "six");
        m.insert(7, "seven");
        m.insert(8, "eight");
        m.insert(9, "nine");
        m
    };
    static ref SPECIAL: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(10, "ten");
        m.insert(11, "eleven");
        m.insert(12, "twelve");
        m.insert(13, "thirteen");
        m.insert(14, "fourteen");
        m.insert(15, "fifteen");
        m.insert(16, "sixteen");
        m.insert(17, "seventeen");
        m.insert(18, "eighteen");
        m.insert(19, "nineteen");
        m
    };
    static ref REST: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(2, "twenty");
        m.insert(3, "thirty");
        m.insert(4, "fourty");
        m.insert(5, "fifty");
        m.insert(6, "sixty");
        m.insert(7, "seventy");
        m.insert(8, "eighty");
        m.insert(9, "ninety");
        m
    };
}

fn solution(n: Input) -> Output {
    // let ones: HashMap<_, _> = HashMap::from_iter([
    //     (1, "one"),
    //     (2, "two"),
    //     (3, "three"),
    //     (4, "four"),
    //     (5, "five"),
    //     (6, "six"),
    //     (7, "seven"),
    //     (8, "eight"),
    //     (9, "nine"),
    // ]);
    // let tens: HashMap<_, _> = HashMap::from_iter([
    //     (10, "ten"),
    //     (11, "eleven"),
    //     (12, "twelve"),
    //     (13, "thirteen"),
    //     (14, "fourteen"),
    //     (15, "fifteen"),
    //     (16, "sixteen"),
    //     (17, "seventeen"),
    //     (18, "eighteen"),
    //     (19, "nineteen"),
    // ]);
    // let rest: HashMap<_, _> = HashMap::from_iter([
    //     (20, "twenty"),
    //     (30, "thirty"),
    //     (40, "fourty"),
    //     (50, "fifty"),
    //     (60, "sixty"),
    //     (70, "seventy"),
    //     (80, "eighty"),
    //     (90, "ninety"),
    // ]);

    for i in 1..=1_000 {
        println!("{}", to_readable(i));
    }
    unimplemented!()
}

fn to_readable(n: u32) -> String {
    let mut result = String::new();
    let digits: Vec<_> = n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    if digits.len() == 4 {
        return "one-thousand".to_string();
    }
    if digits.len() > 2 {
        result.push_str(ONES[&digits[0]]);
        // result.push_str("-hundred and ");
        result.push_str("-hundred");
        if digits[1] == 0 && digits[2] == 0 {
            return result;
        }
        result.push_str(" and ");
    }
    if digits.len() > 1 {
        let x = n % 100;
        if x > 9 && x < 20 {
            result.push_str(SPECIAL[&x]);
            return result;
        // } else if  {
        } else {
            result.push_str(REST[&(x / 10)]);
            if digits.last().unwrap() == &0 {
                return result;
            }
            result.push_str("-");
        };
    }

    result.push_str(ONES[&digits.last().unwrap()]);
    result
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::rstest;
//     #[rstest]
//     #[case(10, 23)]
//     #[case(1000, 233168)]
//     fn solution_test(#[case] input: Input, #[case] expected: Output) {
//         assert_eq!(solution(input), expected)
//     }
// }
