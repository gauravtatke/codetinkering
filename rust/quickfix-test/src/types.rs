pub mod fix_type {
    pub trait FixType<Other=Self> {
        type output;

        fn get(&self) -> Self::output;

        fn set(&mut self, val: Other); // TODO: implement error returning
    }

    pub trait Setter<Other=Self> {
        fn set(&mut self, other: Other);
    }
}

pub mod integer_type {
    //use std::default::Default;
    use types::fix_type::Setter;

    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Int (i64);
    impl Int {
        pub fn new() -> Self {
            Int(0)
        }

        pub fn get(&self) -> i64 {
            self.0
        }
    }

    impl Setter for Int {
        fn set(&mut self, other: Int) {
            self.0 = other.0;
        }
    }

    impl Setter<i32> for Int {
        fn set(&mut self, other: i32) {
            self.0 = other as i64;
        }
    }

    impl Setter<i64> for Int {
        fn set(&mut self, other: i64) {
            self.0 = other;
        }
    }

    impl Setter<u32> for Int {
        fn set(&mut self, other: u32) {
            self.0 = other as i64;
        }
    }

    impl Setter<u64> for Int {
        fn set(&mut self, other: u64) {
            self.0 = other as i64;
        }
    }

    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Length (u32);
    impl Length {
        fn new() -> Self {
            Length(0)
        }

        fn get(&self) -> u32 {
            self.0
        }
    }

    impl Setter for Length {
        fn set(&mut self, other: Length) {
            self.0 = other.0;
        }
    }

    impl Setter<u32> for Length {
        fn set(&mut self, other: u32) {
            self.0 = other;
        }
    }

    // tag number impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct TagNum (u32);
    impl TagNum {
        fn new() -> Self {
            TagNum(0)
        }

        fn get(&self) -> u32 {
            self.0
        }
    }

    impl Setter for TagNum {
        fn set(&mut self, other: TagNum) {
            self.0 = other.0;
        }
    }

    impl Setter<u32> for TagNum {
        fn set(&mut self, other: u32) {
            self.0 = other;
        }
    }

    
    // DayOfMonth impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct DayOfMonth (u8);
    impl DayOfMonth {
        fn new() -> Self {
            DayOfMonth(0)
        }

        fn get(&self) -> u8 {
            self.0
        }
    }

    impl Setter for DayOfMonth {
        fn set(&mut self, other: DayOfMonth) {
            self.0 = other.0;
        }
    }

    impl Setter<u8> for DayOfMonth {
        fn set(&mut self, other: u8) {
            self.0 = other;
        }
    }

    impl Setter<u32> for DayOfMonth {
        fn set(&mut self, other: u32) {
            if other > 31 {
                // TODO: fix with panic
                println!("Invalid DayOfMonth, Out of range = {}", other);
            }
            self.0 = other as u8;
        }
    }


    // SeqNum impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct SeqNum (u64);
    impl SeqNum {
        fn new() -> Self {
            SeqNum(0)
        }

        fn get(&self) -> u64 {
            self.0
        }
    }

    impl Setter for SeqNum {
        fn set(&mut self, other: SeqNum) {
            self.0 = other.0;
        }
    }

    impl Setter<u32> for SeqNum {
        fn set(&mut self, other: u32) {
            self.0 = other as u64;
        }
    }

}

pub mod float_type {
    use types::fix_type::Setter;
    use types::integer_type;

    //Float impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Float (f64);
    impl Float {
        pub fn new() -> Self {
            Float(0.0)
        }

        pub fn get(&self) -> f64 {
            self.0
        }
    }

    impl Setter for Float {
        fn set(&mut self, other: Float) {
            self.0 = other.0;
        }
    }

    impl Setter<integer_type::Int> for Float {
        fn set(&mut self, other: integer_type::Int) {
            self.0 = other.get() as f64;
        }
    }

    impl Setter<u32> for Float {
        fn set(&mut self, other: u32) {
            self.0 = other as f64;
        }
    }

    impl Setter<i32> for Float {
        fn set(&mut self, other: i32) {
            self.0 = other as f64;
        }
    }

    impl Setter<u64> for Float {
        fn set(&mut self, other: u64) {
            self.0 = other as f64;
        }
    }

    impl Setter<i64> for Float {
        fn set(&mut self, other: i64) {
            self.0 = other as f64;
        }
    }

    //Price impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct Price (f64);
    impl Price {
        pub fn new() -> Self {
            Price(0.0)
        }

        pub fn get(&self) -> f64 {
            self.0
        }
    }

    impl Setter for Price {
        fn set(&mut self, other: Price) {
            self.0 = other.0;
        }
    }

    impl Setter<integer_type::Int> for Price {
        fn set(&mut self, other: integer_type::Int) {
            self.0 = other.get() as f64;
        }
    }

    impl Setter<u32> for Price {
        fn set(&mut self, other: u32) {
            self.0 = other as f64;
        }
    }

    impl Setter<i32> for Price {
        fn set(&mut self, other: i32) {
            self.0 = other as f64;
        }
    }

    impl Setter<u64> for Price {
        fn set(&mut self, other: u64) {
            self.0 = other as f64;
        }
    }

    impl Setter<i64> for Price {
        fn set(&mut self, other: i64) {
            self.0 = other as f64;
        }
    }


    //PriceOffset impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct PriceOffset (f64);

    //price can be added or subtra

    // Amt impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct Amt (f64);

    // Percent impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct Percent (f64);

    // Qty impls
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct Qty (f64);
}

pub mod char_type {

    // Chr impls
    pub struct Chr (char);

    // Bool impls
    pub struct Bool (bool);
}

pub mod string_type {}