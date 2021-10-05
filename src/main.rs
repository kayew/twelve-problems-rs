mod lib;

fn main() {
    println!(
        "distance_from_origin(77.0, 77.0) = {:.3}",
        lib::distance_from_origin(77.0, 77.0)
    );
    println!("second_digit_5(151) = {}", lib::second_digit_5(151));
    println!(
        "ends_with_uppercase_letter(\"doG\") = {}",
        lib::ends_with_uppercase_letter("doG")
    );
    println!(
        "pow(2, 3) = {}, pow(2, -3) = {}",
        lib::pow(2, 3),
        lib::pow(-3, 3)
    );
    println!(
        "first_difference(\"abc\", \"axy\") = {}",
        lib::first_difference("abc", "axy")
    );
    println!(
        "most_common_character(\"abcbcdc\") = {}",
        lib::most_common_character("abcbcdc")
    );
    println!(
        "first_divisible_by_77([88, 24, 154, 77]) = {}",
        lib::first_divisible_by_77(vec![88, 24, 154, 77])
    );
    println!("powers_of_two(3) = {:?}", lib::powers_of_two(3));
    println!(
        "max_array([2, 10], [1, 200]) = {:?}",
        lib::max_array(vec![2, 10], vec![1, 200])
    );
    println!(
        "times_occur([1,2], [7,1,2,7,7,7,1,2,7]) = {}",
        lib::times_occur(vec![1, 2], vec![7, 1, 2, 7, 7, 7, 1, 2, 7])
    )
}
