use std::mem;

pub fn endian_swap<T: Copy>(value: T) -> T {
    let mut ret: T = unsafe { mem::zeroed() };
    let source = &value as *const T as *const u8;
    let dest = &mut ret as *mut T as *mut u8;

    for i in 0..mem::size_of::<T>() {
        unsafe {
            *dest.add(i) = *source.add(mem::size_of::<T>() - i - 1);
        }
    }

    ret
}
