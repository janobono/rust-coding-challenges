use std::str::FromStr;

const WEIGHTS: [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];

pub fn validate_isbn(data: &str) -> bool {
    Isbn::from_str(data).is_ok()
}

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum InvalidIsbn {
    TooLong,
    TooShort,
    InvalidCharacter(usize, char),
    InvalidCheckDigit,
}

impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = vec![];

        for (i, c) in s.char_indices() {
            match c {
                '0'..='9' => {
                    let digit = c.to_digit(10).unwrap() as u8;
                    digits.push(digit);
                }
                '-' => continue,
                _ => return Err(InvalidIsbn::InvalidCharacter(i, c)),
            };
        }

        if digits.len() < 13 {
            return Err(InvalidIsbn::TooShort);
        } else if digits.len() > 13 {
            return Err(InvalidIsbn::TooLong);
        }

        if digits[12] != calculate_check_digit(&digits) {
            return Err(InvalidIsbn::InvalidCheckDigit);
        }

        Ok(Isbn {
            raw: s.to_string(),
            digits,
        })
    }
}

fn calculate_check_digit(digits: &[u8]) -> u8 {
    let sum: u32 = digits.iter()
        .zip(WEIGHTS.iter())
        .map(|(&x, &y)| x * y)
        .map(|x| x as u32)
        .sum();

    let check_digit = 10 - (sum % 10);

    match check_digit {
        10 => 0_u8,
        x => x as u8,
    }
}
