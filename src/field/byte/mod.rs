use crate::field::traits::field::*;

#[derive(Copy, Clone)]
pub struct F256(pub u8);

    // generate log and antilog lookup tables
    const fn logexp_f256() -> ([u8; 256], [u8; 256]) {
        let mut log: [u8; 256] = [0;256];
        let mut exp: [u8; 256] = [0;256];
        let g: u16 = 0x03;
        let mut t: u16 = 0x01;
        let mut i: u8 = 0x00;
        while i < u8::MAX{
            exp[i as usize] = t as u8;
            log[t as usize] = i as u8;
            t = mul_f256(t,g);
            i += 1;
        }
        (log,exp)
    }

    // multiply over F256 through shift and xor
    const fn mul_f256(x: u16, y: u16) -> u16 {
        if x == 0 || y == 0 { return 0; }

        // multiply both terms on 2 bytes
        let mut acc: u16 = 0x00;
        let mut i = 7;
        while i >= 0 {
            if (0x01 << i) & y > 0 { 
                if acc == 0 { acc = x << i; }
                else { acc ^= x << i; }
            }
            i -= 1;
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

        acc
    }
    


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

        F256(mul_f256(self.0 as u16,other.0 as u16) as u8)

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

    // lookup method
    fn inv2(&self) -> Self{
       F256(Self::LOGEXP.1[(255 - self.log().0) as usize])
    }
}

impl ExtField for F256{
    
    const LOGEXP: ([u8;256],[u8; 256]) = logexp_f256();

    fn div(&self, other: &Self) -> Option<Self>{
        if other.0 == 0 { return None; }
        if self.0 == 0 { return Some(F256(0)); }
        Some(self.mul(&other.inv2()))
    }

    fn sub(&self, other: &Self) -> Self{
        self.add(other)
    }

    // lookup method
    fn mul2(&self, other: &Self) -> Self{
        if self.0 == 0 || other.0 == 0 { return F256(0); }
        let z: u16 = self.log().0 as u16 + other.log().0 as u16;
        F256((z % 255) as u8).exp()
    }

    fn log(&self) -> Self {
        F256(Self::LOGEXP.0[self.0 as usize])
    }

    fn exp(&self) -> Self {
        F256(Self::LOGEXP.1[self.0 as usize])
    }

    // a^n = g^(n log(a) mod g)
    fn pow(&self, e: u8) -> Self{
        if e == 0 { return F256(1); }
        if self.0 == 0 { return F256(0); }
        let z: usize = ((e as u16 * self.log().0 as u16) % 255) as usize;
        F256(Self::LOGEXP.1[z])
    }

    // a^n = a * a * ... * a (n times)
    fn pow2(&self, e: u8) -> Self{
        if e == 0 { return F256(1); }
        if self.0 == 0 { return F256(0); }
        let mut x: F256 = *self;
        for _i in 1..e{
            x = x.mul2(self);
        }
        x
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

        assert_eq!(F256(255).inv2().mul(&F256(255)).0, 1);
        assert_eq!(F256(255).inv2().mul2(&F256(255)).0, 1);
        assert_eq!(F256(128).inv2().mul(&F256(128)).0, 1);
        assert_eq!(F256(128).inv2().mul2(&F256(128)).0, 1);

        assert_eq!(F256(0).inv2().mul(&F256(0)).0, 0);

    }

    #[test]
    fn inv(){
       // untested
    }

    #[test]
    fn mul2(){

        assert_eq!((F256(0x53).mul2(&F256(0xCA))).0, 0x1);
    }

    #[test]
    fn pow() {
        assert_eq!(F256(123).pow(2).0, F256(123).pow2(2).0);
        assert_eq!(F256(255).pow(3).0, F256(255).pow2(3).0);
        assert_eq!(F256(1).pow(4).0, F256(1).pow2(4).0);

    }
}