// CiUY main validation. The code receives a string and doesn't care what you use
// to separate the digits, it gets the numbers from the string and checks the
// verification digit. So all of these formats are valid: `1.111.111-1`,
// `1_111_111_1`, `1.111.111/1`.
// The validation algorithm is:
// Multiply each digit by 2, 9, 8, 7, 6, 3, 4 one to one
// Then do 10 - (sum mod 10), that's the verification digit
mod ci_uy {
    use regex::Regex;
    use std::char;

    // Cleans up anything in the string that is not a digit and pre-pends a "0"
    // in case it's a 6 digit cedula (excluding the verifying digit)
    pub fn transform(cedula: String) -> String {
        let mut limpia: String;
        let re = Regex::new(r"\D").unwrap();

        limpia = re.replace_all(&cedula, "").to_string();

        if limpia.len() == 7 {
            let mut temp = "0".to_string();
            temp.push_str(&limpia);
            limpia = temp;
        }
        limpia
    }

    pub fn validate(input_string: String) -> bool {
        let cedula = transform(input_string);

        if cedula.len() < 6 {
            return false;
        }

        let mut char_vec: Vec<char> = cedula.chars().collect();
        let valid_vec = vec![2, 9, 8, 7, 6, 3, 4];
        let mut sum_vec = Vec::new();

        let digit = char_vec.pop().unwrap();

        for i in 0..char_vec.len() {
            sum_vec.push(char_vec[i].to_digit(10).unwrap_or(0) * valid_vec[i]);
        }
        let suma: u32 = sum_vec.iter().sum();
        let ver_digit = 10 - (suma % 10);
        digit == char::from_digit(ver_digit, 10).unwrap()
    }
}

fn main() {
    let cedula: String = std::env::args().nth(1).expect("no pattern given");
    let valid = ci_uy::validate(cedula);
    println!("{0}", valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numbers() {
        assert!(ci_uy::validate("11111111".to_string()));
        assert!(ci_uy::validate("12345672".to_string()));
    }

    #[test]
    fn test_validate_numbers_with_dots_and_dashes() {
        assert!(ci_uy::validate("1.111.111-1".to_string()));
        assert!(ci_uy::validate("1-234/567/2".to_string()));
    }

    #[test]
    fn test_doesnae_validate_wrong_numbers() {
        assert!(!ci_uy::validate("1.111.222-1".to_string()));
        assert!(!ci_uy::validate("11112221".to_string()));
    }

    #[test]
    fn test_ci_with_6_digits() {
        assert!(ci_uy::validate("111.111-3".to_string()));
        assert!(ci_uy::validate("1111113".to_string()));
    }

    #[test]
    fn test_doesnae_validate_numbers_smaller_than_100_000() {
        assert!(!ci_uy::validate("12345".to_string()));
        assert!(!ci_uy::validate("17".to_string()));
        assert!(!ci_uy::validate("34.993".to_string()));
    }
}
