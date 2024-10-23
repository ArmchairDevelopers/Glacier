use std::{cell::UnsafeCell, sync::Arc};

use tokio::sync::{Mutex, Semaphore};

// unsafe fn extract_semaphore_from_mutex<T>(mutex: Arc<Mutex<T>>) -> Arc<Semaphore> {
//     // Convert the `Arc` into a raw pointer
//     let raw_mutex: *const Mutex<T> = Arc::into_raw(mutex);
    
//     // Reinterpret it as a raw pointer to `Mutex<()>`
//     let raw_mutex_unit: *const () = raw_mutex as *const ();

//     // Extract the `Mutex` from the pointer and assume control of the `Arc`
//     let mutex_ref: &Mutex<T> = &*(raw_mutex_unit as *const Mutex<T>);

//     // Get the pointer to the inner `Semaphore`
//     let semaphore_ptr: *const Semaphore = &*mutex_ref.semaphore as *const Semaphore;

//     // Convert the raw pointer to `Arc<Semaphore>`
//     Arc::from_raw(semaphore_ptr as *const Semaphore)
// }

// unsafe fn convert_arc_mutex_foo_to_bar(
//     arc_foo: Arc<Mutex<dyn To>>,
//     bar_vtable: *const (),
// ) -> Arc<Mutex<dyn Bar>> {
//     // Convert the `Arc` into a raw pointer
//     let raw_foo: *const Mutex<dyn Foo> = Arc::into_raw(arc_foo);
    
//     // Reinterpret it as a raw pointer to `Mutex<()`
//     let raw_mutex_foo: *const () = raw_foo as *const ();

//     // Extract the `Mutex` from the pointer and assume control of the `Arc`
//     let foo_mutex_ref: &Mutex<dyn Foo> = &*(raw_mutex_foo as *const Mutex<dyn Foo>);
    
//     // Lock the mutex to access the inner `dyn Foo`
//     let lock = foo_mutex_ref.lock().await;

//     // Get the pointer to the trait object (`dyn Foo`)
//     let data_ptr: *const () = &**lock as *const _ as *const ();

//     // Rebuild the `dyn Bar` using the `data_ptr` and the `bar_vtable`
//     let bar_trait_obj: *const dyn Bar = std::mem::transmute((data_ptr, bar_vtable));

//     // Convert the raw pointer to `Arc<Mutex<dyn Bar>>`
//     Arc::from_raw(bar_trait_obj as *const Mutex<dyn Bar>)
// }
