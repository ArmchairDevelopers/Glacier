use glacier_reflect::type_info::TypeObject;

#[repr(C)]
pub struct RawTypeObject {
    pub data_ptr: *mut u8,
    pub vtable_ptr: *mut (),
}

unsafe impl Send for RawTypeObject {}

pub fn data_ptr_from_trait_object(data: &mut dyn TypeObject) -> RawTypeObject {
    let ptr = data as *mut dyn TypeObject;
    let (data_ptr, vtable_ptr): (*mut u8, *mut ()) = unsafe { std::mem::transmute(ptr) };
    RawTypeObject {
        data_ptr,
        vtable_ptr,
    }
}

#[derive(Clone, Copy)]
pub struct SendPtr<T>(pub *mut T);

unsafe impl<T> Send for SendPtr<T> {}
