/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Foo {
    pub _bitfield_1: u8,
    pub __bindgen_align: [u8; 0usize],
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Foo ) ));
    assert_eq! (::std::mem::align_of::<Foo>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Foo ) ));
}
extern "C" {
    #[link_name = "_ZN3Foo4typeEv"]
    pub fn Foo_type(this: *mut Foo) -> ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "_ZN3Foo9set_type_Ec"]
    pub fn Foo_set_type_(this: *mut Foo, c: ::std::os::raw::c_char);
}
extern "C" {
    #[link_name = "_ZN3Foo8set_typeEc"]
    pub fn Foo_set_type(this: *mut Foo, c: ::std::os::raw::c_char);
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}
impl Foo {
    #[inline]
    pub fn type__bindgen_bitfield(&self) -> ::std::os::raw::c_char {
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        let mask = 7u64 as u8;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u8) }
    }
    #[inline]
    pub fn set_type__bindgen_bitfield(&mut self,
                                      val: ::std::os::raw::c_char) {
        let mask = 7u64 as u8;
        let val = val as u8 as u8;
        let mut unit_field_val: u8 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(&self._bitfield_1 as *const _ as
                                                *const u8,
                                            &mut unit_field_val as *mut u8 as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>())
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(&unit_field_val as *const _ as
                                                *const u8,
                                            &mut self._bitfield_1 as *mut _ as
                                                *mut u8,
                                            ::std::mem::size_of::<u8>());
        }
    }
    #[inline]
    pub fn new_bitfield_1(type__bindgen_bitfield: ::std::os::raw::c_char)
     -> u8 {
        ({ 0 } |
             ((type__bindgen_bitfield as u8 as u8) << 0usize) & (7u64 as u8))
    }
    #[inline]
    pub unsafe fn type_(&mut self) -> ::std::os::raw::c_char {
        Foo_type(self)
    }
    #[inline]
    pub unsafe fn set_type_(&mut self, c: ::std::os::raw::c_char) {
        Foo_set_type_(self, c)
    }
    #[inline]
    pub unsafe fn set_type(&mut self, c: ::std::os::raw::c_char) {
        Foo_set_type(self, c)
    }
}
