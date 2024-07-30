#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = *(core::ptr::addr_of!((*this).storage) as *const u8)
            .offset(byte_index as isize);
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val { byte | mask } else { byte & !mask }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = (core::ptr::addr_of_mut!((*this).storage) as *mut u8)
            .offset(byte_index as isize);
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if Self::raw_get_bit(this, i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Date {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Date"][::std::mem::size_of::<Date>() - 3usize];
    ["Alignment of Date"][::std::mem::align_of::<Date>() - 1usize];
};
impl Date {
    #[inline]
    pub fn day(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 5u8) as u8) }
    }
    #[inline]
    pub fn set_day(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn day_raw(this: *const Self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 5u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_day_raw(this: *mut Self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                5u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn month(&self) -> ::std::os::raw::c_uchar {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_month(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn month_raw(this: *const Self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 5usize, 4u8) as u8,
            )
        }
    }
    #[inline]
    pub unsafe fn set_month_raw(this: *mut Self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                5usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn year(&self) -> ::std::os::raw::c_short {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 15u8) as u16) }
    }
    #[inline]
    pub fn set_year(&mut self, val: ::std::os::raw::c_short) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 15u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn year_raw(this: *const Self) -> ::std::os::raw::c_short {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 3usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 9usize, 15u8)
                    as u16,
            )
        }
    }
    #[inline]
    pub unsafe fn set_year_raw(this: *mut Self, val: ::std::os::raw::c_short) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 3usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                9usize,
                15u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        day: ::std::os::raw::c_uchar,
        month: ::std::os::raw::c_uchar,
        year: ::std::os::raw::c_short,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                5u8,
                {
                    let day: u8 = unsafe { ::std::mem::transmute(day) };
                    day as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                5usize,
                4u8,
                {
                    let month: u8 = unsafe { ::std::mem::transmute(month) };
                    month as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                9usize,
                15u8,
                {
                    let year: u16 = unsafe { ::std::mem::transmute(year) };
                    year as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
