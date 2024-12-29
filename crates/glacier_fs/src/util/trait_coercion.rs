use glacier_reflect::type_info::{TypeInfo, TypeInfoData, TypeObject};

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

pub fn vtable_ptr_from_type(info: &'static TypeInfo) -> Option<*mut ()> {
    if let TypeInfoData::Class(class_info) = &info.data {
        let mut ptr = (class_info.functions.create_boxed)();
        let raw = data_ptr_from_trait_object(&mut *ptr);
        return Some(raw.vtable_ptr);
    } else if let TypeInfoData::ValueType(enum_info) = &info.data {
        let functions = &enum_info.functions;
        let mut ptr = (functions.create_boxed)();
        let raw = data_ptr_from_trait_object(&mut *ptr);
        return Some(raw.vtable_ptr);
    }

    None
}

pub fn replace_vtable(data: &mut dyn TypeObject, vtable_ptr: *mut ()) {
    let mut raw = data_ptr_from_trait_object(data);
    unsafe {
        std::ptr::write(&mut raw.vtable_ptr, vtable_ptr);
    }
}

pub fn type_info_cast<'a, T: TypeObject>(
    data: &'a mut dyn TypeObject,
    info: &'static TypeInfo,
) -> Option<&'a mut T> {
    let raw = data_ptr_from_trait_object(data);
    let vtable_ptr = vtable_ptr_from_type(info)?;
    replace_vtable(data, vtable_ptr);
    Some(unsafe { &mut *(raw.data_ptr as *mut T) })
}

#[derive(Debug, Clone, Copy)]
pub struct SendPtr<T>(pub *mut T);

unsafe impl<T> Send for SendPtr<T> {}
