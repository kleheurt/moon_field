use crate::field::traits::field::*;

pub struct F2(pub bool);

impl Field for F2{
    const ZERO: Self = Self(false);
    const ONE: Self = Self(true);

    fn is_zero(&self) -> bool { !self.0 } 

    fn add(&self, other: &Self) -> Self {
        F2(self.0 != other.0)
    }

    fn mul(&self, other: &Self) -> Self{
        F2(! (self.is_zero() || other.is_zero()) )
    } 

    fn neg(&'_ mut self) -> &'_ mut Self{
        self.0 = !self.0;
        self
    }

    fn inv(&self) -> Option<&Self>{
        if self.0 { Some(&F2(true)) } else { None }
    }

}

#[cfg(test)]
mod tests {

    use crate::field::{binary::F2, traits::field::Field};

    #[test]
    fn baseline() {
        assert_eq!(F2(true).0, true);
        assert_eq!(F2(false).0, false);
        assert!(F2(true).is_zero() ^ F2(false).is_zero());
        assert_eq!(F2(true).is_zero(), false);
        assert_eq!(F2(false).is_zero(), true);

        assert_eq!(F2::ONE.0, true);
        assert_eq!(F2::ZERO.0, false);

    }

    #[test]
    fn add(){
        assert_eq!((F2(false).add(&F2(true))).0, true);
        assert_eq!((F2(false).add(&F2(false))).0, false);
        assert_eq!((F2(true).add(&F2(false))).0, true);
        assert_eq!((F2(true).add(&F2(true))).0, false);

    }

    #[test]
    fn mul(){
        assert_eq!((F2(false).mul(&F2(true))).0, false);
        assert_eq!((F2(false).mul(&F2(false))).0, false);
        assert_eq!((F2(true).mul(&F2(false))).0, false);
        assert_eq!((F2(true).mul(&F2(true))).0, true);

    }

    #[test]
    fn neg(){
        assert_eq!(F2(false).neg().0, true);
        assert_eq!(F2(true).neg().0, false);

    }

    #[test]
    fn inv(){
        match F2(false).inv(){
            Some(_x) => panic!("inv 0 ko"),
            None => println!("inv 0 ok"),
        }

        match F2(true).inv(){
            Some(x) => {
                if x.0 == true {println!("inv 1 ok");}
                else {panic!("inv 1 ko");}
            },
            None => panic!("inv 1 ko"),
        }

    }
}