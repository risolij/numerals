use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Numeral {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
    Other = 0,
}

impl From<char> for Numeral {
    fn from(input: char) -> Self {
        match input {
            'I' => Numeral::I,
            'V' => Numeral::V,
            'X' => Numeral::X,
            'L' => Numeral::L,
            'C' => Numeral::C,
            'D' => Numeral::D,
            'M' => Numeral::M,
            _ => Numeral::Other,
        }
    }
}

#[derive(Debug)]
enum ParseNumeralError {
    FailedToParseStr,
}

#[derive(Debug)]
struct Numerals {
    numerals: Vec<Numeral>,
}

impl FromStr for Numerals {
    type Err = ParseNumeralError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numerals: s.to_uppercase().chars().map(Numeral::from).collect(),
        })
    }
}

impl Numerals {
    fn new(numerals: Vec<Numeral>) -> Self {
        Self { numerals }
    }

    fn convert_to_int(&self) -> i32 {
        self.numerals
            .iter()
            .rev()
            .map(|number| *number as i32)
            .scan(0, | state, number: i32 | {
                match number.cmp(&state) {
                    Ordering::Less => Some(-number),
                    Ordering::Equal => Some(number),
                    Ordering::Greater => { *state = number; Some(number) }
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_single_numeral() {
        let i: Numerals = Numerals::new(vec![Numeral::from('I')]);
        assert_eq!(1, i.convert_to_int());
    }

    #[test]
    fn convert_to_sum_2000() {
        let letters = "MM";
        let numerals: Numerals = Numerals::new(letters.chars().map(Numeral::from).collect());

        assert_eq!(2000, numerals.convert_to_int());
    }

    #[test]
    fn convert_to_sum_2005() {
        let letters = "MMV";
        let numbers: Vec<Numeral> = letters.chars().map(Numeral::from).collect();
        let numerals: Numerals = Numerals::new(numbers);

        assert_eq!(2005, numerals.convert_to_int());
    }

    #[test]
    fn convert_all() {
        let letters = "IVXLCDM";
        let numbers: Vec<Numeral> = letters.chars().map(Numeral::from).collect();
        let numerals: Numerals = Numerals::new(numbers);

        assert_eq!(334, numerals.convert_to_int());
    }

    #[test]
    fn can_create_numerals_directly() {
        let numerals: Numerals = Numerals::from_str("MMV").unwrap();
        assert_eq!(2005, numerals.convert_to_int());

        let numerals: Numerals = Numerals::from_str("MMMV").unwrap();
        assert_eq!(3005, numerals.convert_to_int());
    }
}
