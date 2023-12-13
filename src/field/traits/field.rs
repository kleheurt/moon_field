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
    fn inv2(&'_ mut self) -> Self;
}