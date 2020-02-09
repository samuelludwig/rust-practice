fn main() {
    println!("Hello, world!");
}

fn phone_numberify(numbers: &[u8]) -> String {
    match numbers {
        numbers if numbers.len() == 10 => build_number(numbers),
        _ => String::new(),

    }

}

fn build_number(numbers: &[u8]) -> String {
    let first_three = numbers[..3].iter()
        .fold(String::new(), |acc, x| { format!("{}{}", acc, x.to_string()) });
    let second_three = numbers[3..6].iter()
        .fold(String::new(), |acc, x| { format!("{}{}", acc, x.to_string()) });
    let last_four = numbers[6..].iter()
        .fold(String::new(), |acc, x| { format!("{}{}", acc, x.to_string()) });
    format!("({}) {}-{}", first_three, second_three, last_four)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_numberify() {
        let all_zeroes_number_array = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(phone_numberify(&[]), "");
        let first_five_chars = &phone_numberify(&all_zeroes_number_array)[..5];
        assert_eq!(first_five_chars, "(000)");
        let next_five_chars = &phone_numberify(&all_zeroes_number_array)[5..10];
        assert_eq!(next_five_chars, " 000-");
        let all_of_it = &phone_numberify(&all_zeroes_number_array);
        assert_eq!(all_of_it, "(000) 000-0000");
    }
}
