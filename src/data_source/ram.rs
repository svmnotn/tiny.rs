use super::{DataSource, Erd, ErdMap};
use core::{
    any::{Any, TypeId},
    mem, ptr,
};

pub struct Ram<'store, 'mapping> {
    storage: &'store mut [u8],
    mapping: &'mapping [ErdMap],
}

impl<'store, 'mapping> Ram<'store, 'mapping> {
    pub fn new(storage: &'store mut [u8], mapping: &'mapping [ErdMap]) -> Self {
        Self { storage, mapping }
    }
}

impl<'store, 'mapping> DataSource for Ram<'store, 'mapping> {
    fn read<T: 'static + Any>(&self, erd: Erd, value: &mut T) {
        if let Ok(index) = self.mapping.binary_search_by(|(v, _, _)| v.cmp(&erd)) {
            let (_, offset, type_id) = self.mapping[index];
            if type_id == TypeId::of::<T>() {
                unsafe {
                    let src = self.storage.as_ptr().add(offset);
                    let src = mem::transmute(src);
                    ptr::copy_nonoverlapping(src, value, 1);
                }
            } else {
                panic!("Wrong Type");
            }
        } else {
            panic!("Erd Not Found");
        }
    }

    fn write<T: 'static + Any>(&self, erd: Erd, value: &T) {
        if let Ok(index) = self.mapping.binary_search_by(|(v, _, _)| v.cmp(&erd)) {
            let (_, offset, type_id) = self.mapping[index];
            if type_id == TypeId::of::<T>() {
                unsafe {
                    let dst = self.storage.as_ptr().add(offset);
                    let dst = mem::transmute(dst);
                    ptr::copy_nonoverlapping(value, dst, 1);
                }
            } else {
                panic!("Wrong Type");
            }
        } else {
            panic!("Erd Not Found");
        }
    }

    fn has(&self, erd: Erd) -> bool {
        self.mapping
            .binary_search_by(|(v, _, _)| v.cmp(&erd))
            .is_ok()
    }
}
