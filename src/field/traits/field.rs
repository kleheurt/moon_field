pub trait Field: Sized{
    const ZERO: Self;
    const ONE: Self;

    fn is_zero(&self) -> bool;

    fn add(&self, other: &Self) -> Self;

    fn mul(&self, other: &Self) -> Self;

    fn neg(&'_ mut self) -> &'_ mut Self;

    fn inv(&self) -> Option<Self>;

}

pub trait InvField {
    fn inv2(&self) -> Self;
}

pub trait ExtField: Sized {

    const LOGEXP: ([u8;256],[u8; 256]);

    fn div(&self, other: &Self) -> Option<Self>;

    fn sub(&self, other: &Self) -> Self;

    fn mul2(&self, other: &Self) -> Self;

    fn log(&self) -> Self;

    fn pow(&self, e: u8) -> Self;

    fn pow2(&self, e: u8) -> Self;

    fn exp(&self) -> Self;

}
