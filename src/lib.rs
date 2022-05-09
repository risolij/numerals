use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Numeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
    Other,
}

impl Numeral {
    fn from_char(input: char) -> Self {
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

    fn numeral_to_i32(&self) -> i32 {
        match self {
            Numeral::I => 1,
            Numeral::V => 5,
            Numeral::X => 10,
            Numeral::L => 50,
            Numeral::C => 100,
            Numeral::D => 500,
            Numeral::M => 1000,
            Numeral::Other => 0,
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
            numerals: s.to_uppercase().chars().map(Numeral::from_char).collect(),
        })
    }
}

impl Numerals {
    fn new(numerals: Vec<Numeral>) -> Self {
        Self { numerals }
    }

    fn convert_to_int(self) -> i32 {
        let mut total = 0;
        let mut max = 0;

        self.numerals
            .into_iter()
            .rev()
            .map(|numeral| numeral.numeral_to_i32())
            .filter(|x| x != 0)
            .for_each(|number: i32| {
                total += match number.cmp(&max) {
                    Ordering::Less => -number,
                    Ordering::Equal => number,
                    Ordering::Greater => { max = number; number }
                }
            });

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_single_numeral() {
        let i: Numerals = Numerals::new(vec![Numeral::from_char('I')]);
        assert_eq!(1, i.convert_to_int());
    }

    #[test]
    fn convert_to_sum_2000() {
        let letters = "MM";
        let numerals: Numerals = Numerals::new(letters.chars().map(Numeral::from_char).collect());

        assert_eq!(2000, numerals.convert_to_int());
    }

    #[test]
    fn convert_to_sum_2005() {
        let letters = "MMV";
        let numbers: Vec<Numeral> = letters.chars().map(Numeral::from_char).collect();
        let numerals: Numerals = Numerals::new(numbers);

        assert_eq!(2005, numerals.convert_to_int());
    }

    #[test]
    fn convert_all() {
        let letters = "IVXLCDM";
        let numbers: Vec<Numeral> = letters.chars().map(Numeral::from_char).collect();
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
