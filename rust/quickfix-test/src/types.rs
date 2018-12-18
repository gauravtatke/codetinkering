use std::convert::From;
use std::convert::Into;

pub trait FixType {
    type Output;

    fn get_val(&self) -> Self::Output;

    fn set_val<T: Into<Self::Output>>(&mut self, val: T);
}

pub trait Setter<Other = Self> {
    fn set(&mut self, other: Other);
}

pub trait Getter {
    type Output;
    fn get(&self) -> Self::Output;
}


impl<O, I: Getter<Output = O> + Setter<O>> FixType for I {
    type Output = O;

    fn get_val(&self) -> Self::Output {
        self.get()
    }

    fn set_val<T: Into<O>>(&mut self, val: T) {
        self.set(val.into());
    }
}

pub mod integer_type {
    //use std::default::Default;
    use crate::types::{Getter, Setter};
    use crate::types::FixType;

    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Int (i64);

    impl Int {
        pub fn new() -> Self {
            Int(0)
        }
    }

    impl Getter for Int {
        type Output = i64;

        fn get(&self) -> i64 {
            self.0
        }
    }

    impl<T: Into<i64>> From<T> for Int {
        fn from(f: T) -> Int {
            Int(f.into())
        }
    }

    impl<T: Into<i64>> Setter<T> for Int {
        fn set(&mut self, val: T) {
            self.0 = val.into();
        }
    }

    impl Setter for Int {
        fn set(&mut self, other: Int) {
            self.0 = other.0;
        }
    }

    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Length (u32);
    impl Length {
        fn new() -> Self {
            Length(0)
        }
    }

    impl Getter for Length {
        type Output = u32;

        fn get(&self) -> u32 {
            self.0
        }
    }

    impl<T: Into<u32>> From<T> for Length {
        fn from(l: T) -> Length {
            Length(l.into())
        }
    }

    impl Setter for Length {
        fn set(&mut self, other: Length) {
            self.0 = other.0;
        }
    }

    impl<T: Into<u32>> Setter<T> for Length {
        fn set(&mut self, val: T) {
            self.0 = val.into();
        }
    }


    // tag number impls
    #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
    pub struct TagNum (u32);
    impl TagNum {
        pub fn new() -> Self {
            TagNum(0)
        }

        // fn get(&self) -> u32 {
        //     self.0
        // }
    }

    impl Getter for TagNum {
        type Output = u32;

        fn get(&self) -> u32 {
            self.0
        }
    }

    impl<T: Into<u32>> From<T> for TagNum {
        fn from(tn: T) -> TagNum {
            TagNum(tn.into())
        }
    }

    impl<T: Into<u32>> Setter<T> for TagNum {
        fn set(&mut self, val: T) {
            self.0 = val.into();
        }
    }

    impl Setter for TagNum {
        fn set(&mut self, other: TagNum) {
            self.0 = other.0;
        }
    }

    
    // DayOfMonth impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct DayOfMonth (u32);
    impl DayOfMonth {
        fn new() -> Self {
            DayOfMonth(0)
        }
    }

    impl Getter for DayOfMonth {
        type Output = u32;

        fn get(&self) -> u32 {
            self.0
        }
    }

    impl Setter for DayOfMonth {
        fn set(&mut self, other: DayOfMonth) {
            self.0 = other.0;
        }
    }

    impl<T: Into<u32>> From<T> for DayOfMonth {
        fn from(dom: T) -> DayOfMonth {
            DayOfMonth(dom.into())
        }
    }

    impl<T: Into<u32>> Setter<T> for DayOfMonth {
        fn set(&mut self, val: T) {
            let tempval = val.into();
            if tempval > 31 {
                panic!("Day of month can not greater than 31");
            }
            self.0 = tempval;
        }
    }

    // SeqNum impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct SeqNum (u64);
    impl SeqNum {
        fn new() -> Self {
            SeqNum(0)
        }
    }

    impl Getter for SeqNum {
        type Output = u64;

        fn get(&self) -> u64 {
            self.0
        }
    }

    impl Setter for SeqNum {
        fn set(&mut self, other: SeqNum) {
            self.0 = other.0;
        }
    }

    impl<T: Into<u64>> From<T> for SeqNum {
        fn from(sq: T) -> SeqNum {
            SeqNum(sq.into())
        }
    }

    impl<T: Into<u64>> Setter<T> for SeqNum {
        fn set(&mut self, sq: T) {
            self.0 = sq.into();
        }
    }
}

pub mod float_type {
    use crate::types::{Getter, Setter};
    use crate::types::integer_type;

    //Float impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Float (f64);
    impl Float {
        pub fn new() -> Self {
            Float(0.0)
        }
    }

    impl Getter for Float {
        type Output = f64;

        fn get(&self) -> f64 {
            self.0
        }
    }

    impl Setter for Float {
        fn set(&mut self, other: Float) {
            self.0 = other.0;
        }
    }

    impl<T: Into<f64>> From<T> for Float {
        fn from(fl: T) -> Float {
            Float(fl.into())
        }
    }

    impl<T: Into<f64>> Setter<T> for Float {
        fn set(&mut self, other: T) {
            self.0 = other.into();
        }
    }

    //Price impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct Price (f64);
    impl Price {
        pub fn new() -> Self {
            Price(0.0)
        }
    }

    impl Getter for Price {
        type Output = f64;

        fn get(&self) -> f64 {
            self.0
        }
    }

    impl<T: Into<f64>> From<T> for Price {
        fn from(val: T) -> Price {
            Price(val.into())
        }
    }

    impl<T: Into<f64>> Setter<T> for Price {
        fn set(&mut self, val: T) {
            self.0 = val.into();
        }
    }

    impl Setter for Price {
        fn set(&mut self, other: Price) {
            self.0 = other.0;
        }
    }


    //PriceOffset impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct PriceOffset (f64);
    impl PriceOffset {
        pub fn new() -> Self {
            PriceOffset(0.0)
        }
    }

    impl Getter for PriceOffset {
        type Output = f64;

        fn get(&self) -> f64 {
            self.0
        }
    }

    impl<T: Into<f64>> From<T> for PriceOffset {
        fn from(val: T) -> PriceOffset {
            PriceOffset(val.into())
        }
    }

    impl<T: Into<f64>> Setter<T> for PriceOffset {
        fn set(&mut self, val: T) {
            self.0 = val.into();
        }
    }

    impl Setter for PriceOffset {
        fn set(&mut self, other: PriceOffset) {
            self.0 = other.0;
        }
    }

    // Amt impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct Amt (f64);
    impl Amt {
        pub fn new() -> Self {
            Amt(0.0)
        }
    }

    impl Getter for Amt {
        type Output = f64;

        fn get(&self) -> f64 {
            self.0
        }
    }

    impl<T: Into<f64>> From<T> for Amt {
        fn from(val: T) -> Amt {
            Amt(val.into())
        }
    }

    impl<T: Into<f64>> Setter<T> for Amt {
        fn set(&mut self, val: T) {
            self.0 = val.into();
        }
    }

    impl Setter for Amt {
        fn set(&mut self, other: Amt) {
            self.0 = other.0;
        }
    }
    // Percent impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct Percent (f64);
    impl Percent {
        pub fn new() -> Self {
            Percent(0.0)
        }
    }

    impl Getter for Percent {
        type Output = f64;

        fn get(&self) -> f64 {
            self.0
        }
    }

    impl<T: Into<f64>> From<T> for Percent {
        fn from(val: T) -> Percent {
            Percent(val.into())
        }
    }

    impl<T: Into<f64>> Setter<T> for Percent {
        fn set(&mut self, val: T) {
            self.0 = val.into();
        }
    }

    impl Setter for Percent {
        fn set(&mut self, other: Percent) {
            self.0 = other.0;
        }
    }
    // Qty impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct Qty (f64);
    impl Qty {
        pub fn new() -> Self {
            Qty(0.0)
        }
    }

    impl Getter for Qty {
        type Output = f64;

        fn get(&self) -> f64 {
            self.0
        }
    }

    impl<T: Into<f64>> From<T> for Qty {
        fn from(val: T) -> Qty {
            Qty(val.into())
        }
    }

    impl<T: Into<f64>> Setter<T> for Qty {
        fn set(&mut self, val: T) {
            self.0 = val.into();
        }
    }

    impl Setter for Qty {
        fn set(&mut self, other: Qty) {
            self.0 = other.0;
        }
    }

    // Price & Qty can be multiplies to get Amt
    // Price and Price Offset can be added to get AllInPrice
}

pub mod char_type {
    use crate::types::{Getter, Setter};
    // use std::str::FromStr;
    // use std::char::ParseCharError;
    // use std::char::convert::CharErrorKind;

    // Chr impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Chr (char);
    impl Chr {
        fn new() -> Chr {
            Chr(' ')
        }
    }

    impl Getter for Chr {
        type Output = char;

        fn get(&self) -> char {
            self.0
        }
    }

    impl<T: Into<char>> From<T> for Chr {
        fn from(val: T) -> Chr {
            Chr(val.into())
        }
    }

    impl<T: Into<char>> Setter<T> for Chr {
        fn set(&mut self, val: T) {
            let ch = val.into();
            if ch.is_ascii() {
                self.0 = ch;
            } else {
                panic!("value can be only ascii")
            }  
        }
    }

    impl Setter for Chr {
        fn set(&mut self, val: Chr) {
            self.set(val.get()); // should call Into<char> setter method
        }
    }

    // copy paste from std char module
    // impl FromStr for Chr {
    //     type Err = ParseCharError;

    //     fn from_str(s: &str) -> Result<Self, Self::Err> {
    //         let mut chars = s.chars();
    //         match (chars.next(), chars.next()) {
    //             (None, _) => {
    //                 Err(ParseCharError { kind: CharErrorKind::EmptyString })
    //             },
    //             (Some(c), None) => {
    //                 Ok(Chr::from(c))
    //             },
    //             _ => {
    //                 Err(ParseCharError { kind: CharErrorKind::EmptyString })
    //             }
    //         }
    //     }
    // }

    // enum CharErrorKind {
    //     EmptyString,
    //     TooManyChars,
    // }

    // Bool impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Bool (char);

    impl Bool {
        fn new() -> Bool {
            Bool('Y')
        }
    }

    impl Getter for Bool {
        type Output = char;

        fn get(&self) -> char {
            self.0
        }
    }

    impl<T: Into<char>> From<T> for Bool {
        fn from(val: T) -> Bool {
            Bool(val.into())
        }
    }

    impl<T: Into<char>> Setter<T> for Bool {
        fn set(&mut self, val: T) {
            let ch = val.into();
            if !'Y'.eq_ignore_ascii_case(&ch) || !'N'.eq_ignore_ascii_case(&ch) {
                panic!(" Value can only be 'Y' or 'N' ");
            }
            self.0 = ch.to_ascii_uppercase();
        }
    }

    // Commenting below as it is conflicting implementation for Setter
    // impl Setter<bool> for Bool {
    //     fn set(&mut self, val: bool) {
    //         if val {
    //             self.0 = 'Y';
    //         } else {
    //             self.0 = 'N'
    //         }
    //     }
    // }

}

pub mod data_type {
    use crate::types::{Getter, Setter};

    #[derive(Clone, Debug)]
    pub struct Data (Vec<u8>);
}

// pub mod string_type {
//     use types::{Getter, Setter};

//     #[derive(Clone, Debug, PartialEq, PartialOrd)]
//     pub struct Str (String);
//     impl Str {
//         fn new() -> Str {
//             Str(String::from(""))
//         }
//     }
//     impl Getter for Str {
//         type Output = String;
//         fn get(&self) -> String {
//             self.0
//         }
//     }

//     impl<T: Into<String>> From<T> for Str {
//         fn from(val: T) -> Str {
//             Str(val.into())
//         }
//     }

//     impl<T: Into<String>> Setter<T> for Str {
//         fn set(&mut self, other: T) {
//             self.0 = other.into();
//         }
//     }

//     // pub struct MultiValStr (String);

//     // pub struct Country (String);

//     #[derive(Clone, Debug, PartialEq, PartialOrd)]
//     pub struct Currency (String);
//     impl Currency {
//         fn new() -> Currency {
//             Currency(String::from(""))
//         }
//     }

//     impl Getter for Currency {
//         type Output = String;

//         fn get(&self) -> String {
//             self.0
//         }
//     }

//     impl<T: Into<String>> From<T> for Currency {
//         fn from(val: T) -> Currency {
//             Currency(val.into())
//         }
//     }

//     impl<T: Into<String>> Setter<T> for Currency {
//         fn set(&mut self, other: T) {
//             self.0 = other.into();
//         }
//     }

//     // pub struct Exchange (String);

//     // pub struct MonthYear (String);

//     #[derive(Clone, Debug, PartialEq, PartialOrd)]
//     pub struct UTCTimeStamp (String);
//     impl UTCTimeStamp {
//         fn new() -> UTCTimeStamp {
//             UTCTimeStamp(String::from(""))
//         }
//     }

//     impl Getter for UTCTimeStamp {
//         type Output = String;

//         fn get(&self) -> String {
//             self.0
//         }
//     }

//     impl<T: Into<String>> From<T> for UTCTimeStamp {
//         fn from(val: T) -> UTCTimeStamp {
//             UTCTimeStamp(val.into())
//         }
//     }

//     impl<T: Into<String>> Setter<T> for UTCTimeStamp {
//         fn set(&mut self, other: T) {
//             self.0 = other.into();
//         }
//     }
//     // pub struct UTCDate (String);

//     // pub struct UTCTime (String);

//     // pub struct LocalMktDate (String);
// }