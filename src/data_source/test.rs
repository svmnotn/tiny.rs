use super::{DataSource, Erd, Ram};
use crate::erd;
use core::{any::TypeId, mem::size_of};

#[repr(u16)]
enum Erds {
    Test = 0x0001,
    Test2 = 0xBEEF,
    Other = 0xDEAD,
}

const MAPS: &[(Erd, usize, TypeId)] = &[
    (erd!(Test), 0, TypeId::of::<u32>()),
    (erd!(Test2), size_of::<u32>(), TypeId::of::<u128>()),
    (
        erd!(Other),
        size_of::<u32>() + size_of::<u128>(),
        TypeId::of::<u8>(),
    ),
];

static mut STORE: &mut [u8; 21] = &mut [0; 21];

fn make_data_source() -> Ram<'static, 'static> {
    unsafe { Ram::new(STORE, MAPS) }
}

#[test]
fn can_write_first() {
    let data_source = make_data_source();
    let x = 256u32;
    data_source.write(erd!(Test), &x);
    let mut y = 0u32;
    data_source.read(erd!(Test), &mut y);

    assert!(x == y)
}

#[test]
#[should_panic(expected = "Wrong Type")]
fn panics_with_wrong_type() {
    let data_source = make_data_source();
    let x = 255u8;
    data_source.write(erd!(Test), &x);
}

#[test]
#[should_panic(expected = "Erd Not Found")]
fn panics_with_wrong_erd() {
    let data_source = make_data_source();
    let x = 0u8;
    data_source.write(5, &x);
}

#[test]
fn can_write_last() {
    let data_source = make_data_source();
    let x = 42u8;
    data_source.write(erd!(Other), &x);
    let mut y = 0u8;
    data_source.read(erd!(Other), &mut y);

    assert!(x == y)
}
