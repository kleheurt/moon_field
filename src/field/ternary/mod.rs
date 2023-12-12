use crate::field::traits::field::*;

pub struct F3(pub u8);

// 0 is 0
// 2 is 255
// Anything else is 1
impl Field for F3{
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    fn is_zero(&self) ->  bool { self.0 == 0 }

    fn add(&self, other: &Self) -> Self {
        if self.0 == 0 { return F3(other.0); }
        if other.0 == 0 { return F3(self.0); }
        if other.0 != self.0 { return F3(0); }
        if other.0 == 255 { return F3(1); }
        else { return F3(255); }
    }

    fn mul(&self, other: &Self) -> Self{
        if self.0 == 0 || other.0 == 0 { return F3(0); }
        if self.0 == other.0 { return F3(1); }
        else { return F3(255); }
    } 

    fn neg(&'_ mut self) -> &'_ mut Self{
        if self.0 > 0 {
            self.0 = if self.0 == 255 { 1 } else { 255 };
        }
        self
    }

    fn inv(&self) -> Option<Self>{
        if self.0 != 0 { Some(F3(self.0))} else { None } // returning ref to self ?
    }


}

#[cfg(test)]
mod tests {

    use crate::field::{ternary::F3, traits::field::Field};

    #[test]
    fn is_zero(){
        assert!(!F3(255).is_zero());
        assert!(!F3(1).is_zero());
        assert!(!F3(123).is_zero());
        assert!(F3(0).is_zero());
    }

    #[test]
    fn add(){
        assert_eq!((F3(0).add(&F3(0))).0, 0);
        assert_eq!((F3(1).add(&F3(1))).0, 255);
        assert_eq!((F3(255).add(&F3(255))).0, 1);
        assert_eq!((F3(0).add(&F3(1))).0, 1);
        assert_eq!((F3(255).add(&F3(0))).0, 255);
    }

    #[test]
    fn mul(){
        assert_eq!((F3(0).mul(&F3(0))).0, 0);
        assert_eq!((F3(1).mul(&F3(1))).0, 1);
        assert_eq!((F3(255).mul(&F3(255))).0, 1);
        assert_eq!((F3(0).mul(&F3(1))).0, 0);
        assert_eq!((F3(255).mul(&F3(0))).0, 0);
        assert_eq!((F3(255).mul(&F3(1))).0, 255);
    }

    #[test]
    fn neg(){
        assert_eq!(F3(0).neg().0, 0);
        assert_eq!(F3(123).neg().0, 255);
        assert!(F3(255).neg().0 != 0);
        assert!(F3(255).neg().0 != 255);

    }

    #[test]
    fn inv(){
        match F3(0).inv(){
            Some(_x) => panic!("inv 0 ko"),
            None => println!("inv 0 ok"),
        }

        match F3(123).inv(){
            Some(x) => {
                if x.0 > 0 && x.0 < 255 {println!("inv 1 ok");}
                else {panic!("inv 1 ko");}
            },
            None => panic!("inv 1 ko"),
        }
        
        match F3(255).inv(){
            Some(x) => {
                if x.0 == 255 {println!("inv 2 ok");}
                else {panic!("inv 2 ko");}
            },
            None => panic!("inv 2 ko"),
        }
    }


}