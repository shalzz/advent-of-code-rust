fn main() {
    let mut count = 0;
    for num in 246540..787419 {
        if meets_criteria_fold(&num.to_string()) {
            count += 1
        }
    }

    println!("Number of possible passwords: {}", count);
}

#[allow(unused)]
fn meets_criteria(num: &str) -> bool {
    let mut iter = num.chars().peekable();
    let mut double = false;
    while let Some(current) = iter.next() {
        match iter.peek() {
            Some(&next) if current > next => return false,
            Some(&next) if current == next => double = true,
            _ => continue,
        };
    }

    double
}

#[allow(unused)]
fn meets_criteria_fold(num: &str) -> bool {
    num.chars()
        .zip(num.chars().skip(1))
        .fold(Some(false), |acc, (current, next)| {
            if current > next {
                return None;
            } else if let Some(val) = acc {
                // keep the val true if we ever get a double digit
                return Some(val || current == next);
            }
            acc
        })
        .unwrap_or(false)
}

// digit <= next digit; all
// and
// digit == next digit; any
#[allow(unused)]
fn meets_criteria_iter(num: &str) -> bool {
    num.chars()
        .zip(num.chars().skip(1))
        .all(|(digit, next)| digit <= next)
        && num
            .chars()
            .zip(num.chars().skip(1))
            .any(|(digit, next)| digit == next)
}

#[test]
pub fn tests_part1_0() {
    assert_eq!(meets_criteria("111111"), true);
    assert_eq!(meets_criteria_iter("111111"), true);
    assert_eq!(meets_criteria_fold("111111"), true);
}

#[test]
pub fn tests_part1_1() {
    assert_eq!(meets_criteria("112345"), true);
    assert_eq!(meets_criteria_iter("112345"), true);
    assert_eq!(meets_criteria_fold("112345"), true);
}

#[test]
pub fn tests_part1_2() {
    assert_eq!(meets_criteria("223450"), false);
    assert_eq!(meets_criteria_iter("223450"), false);
    assert_eq!(meets_criteria_fold("223450"), false);
}

#[test]
pub fn tests_part1_3() {
    assert_eq!(meets_criteria("123789"), false);
    assert_eq!(meets_criteria_iter("123789"), false);
    assert_eq!(meets_criteria_fold("123789"), false);
}

#[test]
pub fn tests_part1_4() {
    assert_eq!(meets_criteria("776631"), false);
    assert_eq!(meets_criteria_iter("776631"), false);
    assert_eq!(meets_criteria_fold("776631"), false);
}

#[test]
pub fn tests_part1_5() {
    assert_eq!(meets_criteria("392268"), false);
    assert_eq!(meets_criteria_iter("392268"), false);
    assert_eq!(meets_criteria_fold("392268"), false);
}
