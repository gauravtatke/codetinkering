pub mod fix_types {
    use std::ops;

    pub trait FixType {}
    pub trait IntType {}

    pub trait CharType {}

    pub trait FloatType {}

    pub trait StringType {}

    pub mod character {
        pub struct Char (char);
        pub struct Bool (char);

        impl super::CharType for Char {}
        impl super::CharType for Bool {}

        impl super::ops::Deref for Char {
            type Target = char;
            fn deref(&self) -> &char {
                &self.0
            }
        }

        impl super::ops::DerefMut for Char {
            // type Target = char;
            fn deref_mut(&mut self) -> &mut char {
                &mut self.0
            }
        }

        impl super::ops::Deref for Bool {
            type Target = char;
            fn deref(&self) -> &char {
                &self.0
            }
        }

        impl super::ops::DerefMut for Bool {
            // type Target = char;
            fn deref_mut(&mut self) -> &mut char {
                &mut self.0
            }
        }
    }

    pub mod integer {
        pub struct Int (u64);

        impl super::IntType for Int {}
        impl super::ops::Deref for Int {
            type Target = u64;
            fn deref(&self) -> &u64 {
                &self.0
            }
        }

        impl super::ops::DerefMut for Int {
            // type Target = u64;
            fn deref_mut(&mut self) -> &mut u64 {
                &mut self.0
            }
        }

        pub struct DayOfMonth (u8);

        impl super::IntType for DayOfMonth {}
        impl super::ops::Deref for DayOfMonth {
            type Target = u8;
            fn deref(&self) -> &u8 {
                &self.0
            }
        }

        impl super::ops::DerefMut for DayOfMonth {
            // type Target = u8;
            fn deref_mut(&mut self) -> &mut u8 {
                &mut self.0
            }
        }


        pub struct Len (u32);

        impl super::IntType for Len {}
        impl super::ops::Deref for Len {
            type Target = u32;
            fn deref(&self) -> &u32 {
                &self.0
            }
        }

        impl super::ops::DerefMut for Len {
            // type Target = u32;
            fn deref_mut(&mut self) -> &mut u32 {
                &mut self.0
            }
        }

        pub struct SeqNum (u64);

        impl super::IntType for SeqNum {}
        impl super::ops::Deref for SeqNum {
            type Target = u64;
            fn deref(&self) -> &u64 {
                &self.0
            }
        }

        impl super::ops::DerefMut for SeqNum {
            // type Target = u64;
            fn deref_mut(&mut self) -> &mut u64 {
                &mut self.0
            }
        }

        pub struct TagNum (u32);

        impl super::IntType for TagNum {}
        impl super::ops::Deref for TagNum {
            type Target = u32;
            fn deref(&self) -> &u32 {
                &self.0
            }
        }

        // impl super::ops::DerefMut for TagNum {
        //     type Target = u32;
        //     fn deref_mut(&mut self) -> &mut u32 {
        //         &mut self.0
        //     }
        // }

    }

    pub mod float {
        pub struct Float (f64);
        impl super::FloatType for Float {}
        impl super::ops::Deref for Float {
            type Target = f64;
            fn deref(&self) -> &f64 {
                &self.0
            }
        }

        impl super::ops::DerefMut for Float {
            // type Target = f64;
            fn deref_mut(&mut self) -> &mut f64 {
                &mut self.0
            }
        }

        pub struct Amt (f64);
        impl super::FloatType for Amt {}
        impl super::ops::Deref for Amt {
            type Target = f64;
            fn deref(&self) -> &f64 {
                &self.0
            }
        }

        impl super::ops::DerefMut for Amt {
            // type Target = f64;
            fn deref_mut(&mut self) -> &mut f64 {
                &mut self.0
            }
        }

        pub struct Percent (f64);
        impl super::FloatType for Percent {}
        impl super::ops::Deref for Percent {
            type Target = f64;
            fn deref(&self) -> &f64 {
                &self.0
            }
        }

        impl super::ops::DerefMut for Percent {
            // type Target = f64;
            fn deref_mut(&mut self) -> &mut f64 {
                &mut self.0
            }
        }

        pub struct Price (f64);
        impl super::FloatType for Price {}
        impl super::ops::Deref for Price {
            type Target = f64;
            fn deref(&self) -> &f64 {
                &self.0
            }
        }

        impl super::ops::DerefMut for Price {
            // type Target = f64;
            fn deref_mut(&mut self) -> &mut f64 {
                &mut self.0
            }
        }

        pub struct PriceOffSet (f64);
        impl super::FloatType for PriceOffSet {}
        impl super::ops::Deref for PriceOffSet {
            type Target = f64;
            fn deref(&self) -> &f64 {
                &self.0
            }
        }

        impl super::ops::DerefMut for PriceOffSet {
            // type Target = f64;
            fn deref_mut(&mut self) -> &mut f64 {
                &mut self.0
            }
        }

        pub struct Qty (f64);
        impl super::FloatType for Qty {}
        impl super::ops::Deref for Qty {
            type Target = f64;
            fn deref(&self) -> &f64 {
                &self.0
            }
        }

        impl super::ops::DerefMut for Qty {
            // type Target = f64;
            fn deref_mut(&mut self) -> &mut f64 {
                &mut self.0
            }
        }
    }

    // pub mod string {
    //     pub struct Str (String);
    //     impl super::StringType for Str {}

    //     impl super::ops::Deref for Str {
    //         type Target = &'a String;
    //         fn deref(&self) -> &'a String {
    //             &'a self.0
    //         }
    //     }

    //     impl super::ops::DerefMut for Str {
    //         // type Target = String;
    //         fn deref_mut(&mut self) -> &mut String {
    //             &mut self.0
    //         }
    //     }
    // }
}