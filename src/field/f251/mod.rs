use crate::field::traits::field::*;

pub struct F251(pub u8);

/**
 * Largest prime field to fit on a byte
 * Using modular arithmetic on two bytes
 */
impl Field for F251{
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    fn is_zero(&self) -> bool { self.0 == 0 } 

    fn add(&self, other: &Self) -> Self {
        F251(((self.0 as u16 + other.0 as u16) % 251) as u8)
    }

    fn mul(&self, other: &Self) -> Self{
        F251(((self.0 as u16 * other.0 as u16) % 251) as u8) 
    } 

    fn neg(&'_ mut self) -> &'_ mut Self{
        self.0 = (251 - self.0) % 251;
        self
    }

    // brute force search
    fn inv(&self) -> Option<Self>{
        if self.0 == 0  { return None; }
        else {
           for i in 0..=250{
                if self.mul(&F251(i)).0 == 1 { return Some(F251(i)); }
            }
            return None;
        }
    }

}

#[cfg(test)]
mod tests {

    use crate::field::{f251::F251, traits::field::Field};

    #[test]
    fn is_zero(){
        assert!(!F251(1).is_zero());
        assert!(!F251(250).is_zero());
        assert!(F251(0).is_zero());
    }

    #[test]
    fn add(){
        assert_eq!((F251(0).add(&F251(1))).0, 1);
        assert_eq!((F251(0).add(&F251(0))).0, 0);
        assert_eq!((F251(1).add(&F251(0))).0, 1);
        assert_eq!((F251(1).add(&F251(1))).0, 2);
        assert_eq!((F251(250).add(&F251(1))).0, 0);
        assert_eq!((F251(250).add(&F251(2))).0, 1);
        assert_eq!((F251(250).add(&F251(250))).0, 249);
        assert_eq!((F251(125).add(&F251(126))).0, 0);



    }

    #[test]
    fn mul(){
        assert_eq!((F251(0).mul(&F251(1))).0, 0);
        assert_eq!((F251(0).mul(&F251(0))).0, 0);
        assert_eq!((F251(1).mul(&F251(0))).0, 0);
        assert_eq!((F251(250).mul(&F251(1))).0, 250);
        assert_eq!((F251(12).mul(&F251(12))).0, 144);
        assert_eq!((F251(128).mul(&F251(2))).0, 5);
        assert_eq!((F251(250).mul(&F251(250))).0, 1);


    }

    #[test]
    fn neg(){
        assert_eq!(F251(0).neg().0, 0);
        assert_eq!(F251(1).neg().0, 250);
        assert_eq!(F251(128).neg().0, 123);
        assert_eq!(F251(200).neg().0, 51);
        assert_eq!(F251(51).neg().0, 200);

    }

    #[test]
    fn inv(){
        match F251(0).inv(){
            Some(_x) => panic!("inv 0 ko"),
            None => println!("inv 0 ok"),
        }

        match F251(1).inv(){
            Some(x) => {
                if x.0 == 1 {println!("inv 1 ok");}
                else {panic!("inv 1 ko");}
            },
            None => panic!("inv 1 ko"),
        }

        match F251(250).inv(){
            Some(x) => {
                if x.0 == 250 {println!("inv 250 ok");}
                else {panic!("inv 250 ko");}
            },
            None => panic!("inv 250 ko"),
        }

        match F251(252).inv(){
            Some(x) => {
                if x.0 == 1 {println!("inv 252 ok");}
                else {panic!("inv 252 ko");}
            },
            None => panic!("inv 252 ko"),
        }
    }
}