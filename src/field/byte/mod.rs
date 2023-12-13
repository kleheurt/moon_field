use crate::field::traits::field::*;

#[derive(Copy, Clone)]
pub struct F256(pub u8);

/**
 * full byte storage
 * multiplication on two bytes
 */
impl Field for F256{
    const ZERO: Self = Self(0x00);
    const ONE: Self = Self(0x01);

    fn is_zero(&self) -> bool { self.0 == 0 }

    fn add(&self, other: &Self) -> Self {
        F256(self.0 ^ other.0)
    }

    // shift and xor method
    fn mul(&self, other: &Self) -> Self{
        // multiply both terms on 2 bytes
        let x = self.0 as u16;
        let mut acc: u16 = 0x00;
        for i in (0..=7).rev() {
            if (0x01 << i) & other.0 > 0 { 
                if acc == 0 { acc = x << i; }
                else { acc ^= x << i; }
            }
        }

        // modulo (x^8 + x^4 + x^3 + x + 1) = 0x11B
        // ie reducing polynomial for GF(2^8)
        while acc >= 0x01 << 8{
            let mut p: u16 = 0x11B;
            while acc ^ p > p {
                p = p << 1;
            }
            acc ^= p;
        }

        // downcasting to a byte
        F256(acc as u8)
    } 

    // subtraction and addition are identical in GF(2^8)
    // so 0 - x == x
    fn neg(&'_ mut self) -> &'_ mut Self{
        self
    }

    // brute force search
    fn inv(&self) -> Option<Self>{
        for i in 0..=u8::MAX{
            if self.mul(&F256(i)).0 == 1 { return Some(F256(i)); }
        }
        return None;
    }

}

impl InvField for F256{
    
    // Itoh-Tsujii inversion algorithm
    fn inv2(&'_ mut self) -> Self{

        // compute = self^r 
        // with r = (2^8 - 1) / (2 - 1) = 255
        let mut a: F256 = *self;
        for _i in 0..253 {
            a = a.mul(self);
        }

        // in GF(2) inversion is a NOOP
        // hence we skip the rest and return
        a
    }
}


#[cfg(test)]
mod tests {

    use crate::field::{byte::F256, traits::field::*};

    #[test]
    fn is_zero(){
        assert!(!F256(1).is_zero());
        assert!(F256(0).is_zero());
    }

    #[test]
    fn add(){
        assert_eq!((F256(5).add(&F256(5))).0, 0x00);
        assert_eq!((F256(12).add(&F256(11))).0, 7);
        assert_eq!((F256(250).add(&F256(255))).0, 5);
        assert_eq!((F256(250).add(&F256(137))).0, 115);
        assert_eq!((F256(0x53).add(&F256(0xCA))).0, 0x99);
    }

    #[test]
    fn mul(){
        assert_eq!((F256(5).mul(&F256(5))).0, 17);
        assert_eq!((F256(12).mul(&F256(11))).0, 116);
        assert_eq!((F256(250).mul(&F256(255))).0, 61);
        assert_eq!((F256(250).mul(&F256(137))).0, 184);
        assert_eq!((F256(0x53).mul(&F256(0xCA))).0, 0x1);
       
    }

    #[test]
    fn neg(){
        assert_eq!(F256(0).neg().0, 0);
        assert_eq!(F256(1).neg().0, 1);
        assert_eq!(F256(128).neg().add(&F256(128)).0, 0);
        assert_eq!(F256(200).neg().add(&F256(200)).0, 0);
        assert_eq!(F256(56).neg().add(&F256(56)).0, 0);
    }

    #[test]
    fn inv2(){

        assert_eq!(F256(1).inv2().mul(&F256(1)).0, 1);
        assert_eq!(F256(255).inv2().mul(&F256(255)).0, 1);
        assert_eq!(F256(128).inv2().mul(&F256(128)).0, 1);
        assert_eq!(F256(0).inv2().mul(&F256(0)).0, 0);

    }

    #[test]
    fn inv(){
       // untested
    }
}