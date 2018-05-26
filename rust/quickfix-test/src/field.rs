pub mod field {
    use types::*;
    //use types::{Ch}

    pub trait Field {
        type tag;
        type value;

        fn tag(&self) -> &Self::tag;
        fn value(&self) -> &Self::value;
        fn value_mut(&mut self) -> &mut Self::value;
        fn set_value(&mut self, val: Self::other_val);
        //fn new() -> Self;
    }

    pub mod char_field {
        use types::fix_types::CharType;
        pub struct CharField<T: CharType> {
            tag: integer::TagNum,
            value: T,
        }

        impl<T: CharType> Field for CharField {
            type tag = integer::TagNum;
            type value = character::Char;

            pub fn tag(&self) -> &Self::tag {
                &self.tag
            }

            pub fn value(&mut self) -> &Self::value {
                &self.value
            }

            pub fn value_mut(&mut self) -> &mut Self::value {
                &mut self.value
            }

            pub fn set_value(&mut self, val: Self::value) {
                self.value = val;
            }
        }

        impl CharField {
            pub fn new() -> CharField {
                CharField {
                    tag: 0,
                    value: 'a',
                }
            }
        }
    }

    pub mod int_field {
        pub struct IntField<T: IntType> {
            tag: integer::TagNum,
            value: T,
        }

        impl IntField {
            pub fn new() -> IntField {
                IntField {
                    tag: 0,
                    value: 0,
                }
            }
        }

        impl<T: IntType> Field for IntField {
            type tag = integer::TagNum;
            type value = integer::Int;

            pub fn tag(&self) -> &Self::tag {
                &self.tag
            }

            pub fn value(&mut self) -> &Self::value {
                &self.value
            }

            pub fn value_mut(&mut self) -> &mut Self::value {
                &mut self.value
            }

            pub fn set_value(&mut self, val) {
                self.value = val;
            }
        }
    } 
}

