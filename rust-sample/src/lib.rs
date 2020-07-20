use std::convert::From;

#[derive(Debug, PartialEq)]
enum MyNumeral {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

impl MyNumeral {
    fn numeral(&self) -> &str {
        match *self {
            MyNumeral::One   => "one",
            MyNumeral::Two   => "two",
            MyNumeral::Three => "three",
            MyNumeral::Four  => "four",
            MyNumeral::Five  => "five",
            MyNumeral::Six   => "six",
            MyNumeral::Seven => "seven",
            MyNumeral::Eight => "eight",
            MyNumeral::Nine  => "nine",
            MyNumeral::Zero  => "zero",
        }
    }
    fn number(&self) -> u8 {
        match *self {
            MyNumeral::One   => 1,
            MyNumeral::Two   => 2,
            MyNumeral::Three => 3,
            MyNumeral::Four  => 4,
            MyNumeral::Five  => 5,
            MyNumeral::Six   => 6,
            MyNumeral::Seven => 7,
            MyNumeral::Eight => 8,
            MyNumeral::Nine  => 9,
            MyNumeral::Zero  => 0,
        }
    }
}


#[derive(Debug)]
enum ErrorBC {
  ErrorB(ErrorB),
  ErrorC(ErrorC),
}
#[derive(Debug)]
enum ErrorC {
    NotOneDigitNumber { number: u8 },
}
impl From<ErrorC> for ErrorBC {
    fn from(error: ErrorC) -> Self {
        ErrorBC::ErrorC(error)
    }
}
// number => enum に変換
fn convert_one_digit_number_to_numeral(number: u8) -> Result<MyNumeral, ErrorC> {
    match number {
        1 => Ok(MyNumeral::One),
        2 => Ok(MyNumeral::Two),
        3 => Ok(MyNumeral::Three),
        4 => Ok(MyNumeral::Four),
        5 => Ok(MyNumeral::Five),
        6 => Ok(MyNumeral::Six),
        7 => Ok(MyNumeral::Seven),
        8 => Ok(MyNumeral::Eight),
        9 => Ok(MyNumeral::Nine),
        0 => Ok(MyNumeral::Zero),
        _ => Err(ErrorC::NotOneDigitNumber { number: number }),
    }
}

// 後々エラーを発見したくて
// '2' => 22はわざと
// '3' => ErrorB::CouldNotConvertはわざと
#[derive(Debug)]
enum ErrorB {
    CouldNotConvert,
}
impl From<ErrorB> for ErrorBC {
    fn from(error: ErrorB) -> Self {
        ErrorBC::ErrorB(error)
    }
}
// '1桁の数字' => 1桁の数値 に変換
fn convert_one_digit_char_to_number(c: char) -> Result<u8, ErrorB> {
    match c {
        '1' => Ok(1),
        '2' => Ok(22),
        '3' => Err(ErrorB::CouldNotConvert),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        '0' => Ok(0),
        _ => Err(ErrorB::CouldNotConvert),
    }
}

#[derive(Debug)]
enum ErrorA {
    NotPositiveNumber,
}
fn convert_positive_number_to_numeral_list(number: i32) -> Result<Vec<Result<MyNumeral, ErrorBC>>, ErrorA> {
    if number <= 0 {
        return Err(ErrorA::NotPositiveNumber);
    }
    Ok(number
        .to_string()
        .chars()
        .map(|c| convert_one_digit_char_to_number(c))        // ErrorBが混ざる可能性がある
        .map(|n| match n {
          Err(why) => Err(why),
          Ok(n) => convert_one_digit_number_to_numeral(n),   // ErrorCが混ざる可能性がある
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(MyNumeral::One.numeral(), "one");
        assert_eq!(MyNumeral::Two.number(), 2);
    }

    #[test]
    fn can_convert_1_to_one() {
        match convert_one_digit_number_to_numeral(1) {
            Ok(MyNumeral::One) => assert!(true),
            _ => assert!(false, "テスト失敗"),
        }
    }

    #[test]
    fn can_not_convert_10() {
        match convert_one_digit_number_to_numeral(10) {
            Err(ErrorC::NotOneDigitNumber { number: _ }) => assert!(true),
            _ => assert!(false, "テスト失敗"),
        }
    }

    #[test]
    fn can_convert_char_0_to_number_0() {
        match convert_one_digit_char_to_number('0') {
            Ok(0) => assert!(true),
            _ => assert!(false, "テスト失敗"),
        }
    }

    #[test]
    fn can_not_convert_not_one_digit_char_to_number() {
        match convert_one_digit_char_to_number('a') {
            Err(ErrorBC::CouldNotConvert) => assert!(true),
            _ => assert!(false, "テスト失敗"),
        }
    }


    #[test]
    fn can_convert_1234_to_one_two_three_four() {
        let result = convert_positive_number_to_numeral_list(1234);
        println!("===========================");
        println!("{:?}", result);
        println!("===========================");
    }
}
