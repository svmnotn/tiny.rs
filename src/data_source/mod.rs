use core::any::{Any, TypeId};

#[macro_export]
macro_rules! erd {
    ($name:ident) => {
        Erds::$name as Erd
    };
}

mod ram;
pub use ram::Ram;
#[cfg(test)]
mod test;

pub type Erd = u16;
pub type ErdMap = (Erd, usize, TypeId);

pub trait DataSource {
    fn read<T: 'static + Any>(&self, erd: Erd, value: &mut T);
    fn write<T: 'static + Any>(&self, erd: Erd, value: &T);
    fn has(&self, erd: Erd) -> bool;
}
