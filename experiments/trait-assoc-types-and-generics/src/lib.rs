use std::marker::PhantomData;

pub struct Gen<T> {
    _val: PhantomData<T>,
}

pub trait WithAssocType {
    type T;
}

impl WithAssocType for Gen<u8> {
    type T = u8;
}

impl WithAssocType for Gen<u16> {
    type T = u16;
}
