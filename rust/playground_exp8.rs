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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    pub struct Int (i64);

    impl Int {
        pub fn new<T: Into<i64>>(val: T) -> Self {
            Int(val.into())
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Float (f64);
    impl Float {
        pub fn new<T: Into<f64>>(val: T) -> Self {
            Float(val.into())
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
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Price (f64);
    impl Price {
        pub fn new<T: Into<f64>>(val: T) -> Self {
            Price(val.into())
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
    
#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct GenericField<T: FixType> {
    tag: u32,
    value: T
}

impl<T: FixType> GenericField<T> {
    fn new(tag: u32, val: T) -> Self {
        Self {
            tag: tag,
            value: val
        }
    }
}
    
fn main() {
    let mut prval = Price::new(19.2);
    let mut intval = Int::new(10);
    let mut int_fld = GenericField::new(38, intval);
    let mut float_fld = GenericField::new(44, prval);
    println!("Hello, world!");
    println!("type 1 = {:?}", intval);
    println!("type 2 = {:?}", prval);
    println!("field 1 = {:?}", int_fld);
    println!("field 2 = {:?}", float_fld);
    prval.set_val(87.45321);
    intval.set_val(6796);
    println!("type 1 = {:?}", intval);
    println!("type 2 = {:?}", prval);
    println!("field 1 = {:?}", int_fld);
    println!("field 2 = {:?}", float_fld);
    let new_prval = prval.get_val();
    let new_intval = intval.get_val();
    println!("new type 1 = {:?}", new_intval);
    println!("new type 2 = {:?}", new_prval);
    
}