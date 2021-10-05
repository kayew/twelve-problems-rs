pub fn distance_from_origin(x: f32, y: f32) -> f32 {
    let delta_x: f32 = x * x;
    let delta_y: f32 = y * y;
    (delta_x + delta_y).sqrt()
}

pub fn second_digit_5(input: i32) -> bool {
    let num = (input / 10) % 10;
    if num == 0 {
        return false;
    }
    num % 5 == 0
}

pub fn ends_with_uppercase_letter(input: &str) -> bool {
    if input == "" {
        return false;
    }
    let result = char::is_uppercase(input.chars().nth(input.len() - 1).unwrap());
    result
}

pub fn pow(num: i32, power: i32) -> f32 {
    let mut result: f32 = 1.0;
    if power == 0 {
        return 1.0;
    }
    if power > 0 {
        for _ in 0..power {
            result *= num as f32
        }
    } else {
        for _ in 0..(power * -1) {
            result /= num as f32
        }
    }
    result
}

pub fn first_difference(one: &str, two: &str) -> i32 {
    let length = one.len();
    if one == two {
        return -1;
    }
    for i in 0..length {
        if one.chars().nth(i) != two.chars().nth(i) {
            return i as i32;
        }
    }
    -1
}

pub fn most_common_character(input: &str) -> char {
    let mut current_char;
    let mut max_char: char = '?';
    let mut current_count;
    let mut max_count: i32 = 0;

    for i in 0..input.len() {
        current_char = input.chars().nth(i).unwrap();
        current_count = 0;
        for j in 0..input.len() {
            if input.chars().nth(j).unwrap() == current_char {
                current_count += 1;
            }
        }

        if current_count > max_count {
            max_count = current_count;
            max_char = current_char;
        }
    }

    max_char
}

pub fn first_divisible_by_77(numbers: Vec<i32>) -> i32 {
    for i in numbers {
        if i % 77 == 0 {
            return i;
        }
    }

    -1
}

pub fn powers_of_two(max_exp: i32) -> Vec<i32> {
    if max_exp < 0 {
        return vec![];
    }

    let mut result: Vec<i32> = vec![];
    let mut current_val: i32 = 1;

    for _ in 0..(max_exp + 1) {
        result.push(current_val);
        current_val *= 2;
    }

    result
}

pub fn max_array(one: Vec<i32>, two: Vec<i32>) -> Vec<i32> {
    let length = one.len();

    let mut arr = vec![0; length];

    for i in 0..length {
        if one[i] > two[i] {
            arr[i] = one[i];
        } else {
            arr[i] = two[i];
        }
    }

    arr
}

pub fn times_occur(shorter: Vec<i32>, longer: Vec<i32>) -> i32 {
    let mut total: i32 = 0;

    for i in 0..=(longer.len() - shorter.len()) {
        for j in 0..shorter.len() {
            if shorter[j] != longer[i + j] {
                total -= 1;
                break;
            }
        }
        total += 1;
    }

    total
}

pub fn double_double(input: Vec<&str>) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();

    for i in 0 .. input.len() {
        let current_word: &str = input.get(i).unwrap();
        if current_word.eq("double") {
            out.push(current_word);
            out.push(current_word);
        } else {
            out.push(current_word);
        }
    }

    out
}

pub fn three_character_strings(input: &str) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    
    if input.len() == 3 {
        out.push(input);
        return out;
    } else if input.len() < 3 {
        return out
    }

    for i in 0 .. (input.len() - 2) {
        out.push(&input[i..i+3])
    }

    out

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_from_origin() {
        assert_eq!(1.0, distance_from_origin(-1.0, 0.0));
        assert_eq!(108.89445, distance_from_origin(77.0, 77.0));
        assert_eq!(5.0, distance_from_origin(-3.0, -4.0));
        assert_eq!(11.18034, distance_from_origin(5.0, -10.0));
    }

    #[test]
    fn test_second_digit_5() {
        assert_eq!(true, second_digit_5(50));
        assert_eq!(true, second_digit_5(55));
        assert_eq!(true, second_digit_5(450));
        assert_eq!(true, second_digit_5(11251));
        assert_eq!(true, second_digit_5(555555));
        assert_eq!(false, second_digit_5(555505));
        assert_eq!(false, second_digit_5(5));
        assert_eq!(false, second_digit_5(230));
    }

    #[test]
    fn test_ends_with_uppercase_letter() {
        assert_eq!(true, ends_with_uppercase_letter("doG"));
        assert_eq!(false, ends_with_uppercase_letter("dog"));
        assert_eq!(true, ends_with_uppercase_letter("HELLO"));
        assert_eq!(false, ends_with_uppercase_letter("hello"));
        assert_eq!(false, ends_with_uppercase_letter(""));
    }

    #[test]
    fn test_pow() {
        assert_eq!(8.0, pow(2,3));
        assert_eq!(1.0, pow(1,100));
        assert_eq!(-27.0, pow(-3, 3));
        assert_eq!((5 * 5 * 5 * 5) as f32, pow(5, 4));
        let num = pow(5, -4);
        assert_eq!(true, num > 0.00159 && num < 0.00161);
        assert_eq!(1.0, pow(100, 0));
        assert_eq!((1.0 / (9.0 * 9.0)), pow(9, -2));
    }

    #[test]
    fn test_double_double() {
        assert_eq!(vec!["foo", "double", "double"], double_double(vec!["foo", "double"]));
        let empty: Vec<&str> = Vec::new();
        assert_eq!(empty, double_double(vec![]));
    }

    #[test]
    fn test_three_characer_strings() {
        assert_eq!(vec!["hel", "ell", "llo"], three_character_strings("hello"))
    }
}
