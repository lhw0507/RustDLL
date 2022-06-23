//#![feature(rustc_private)]
//extern crate libc;
#[repr(C)]
pub struct RustObject {
    Key: i32,
    dDiameter : f64,
    enType : i32,
    // Other members...
}

#[no_mangle]
pub extern "C" fn struct_from_rust() -> RustObject{
    RustObject{Key : 3, dDiameter : 5.0, enType : 2}
}



#[no_mangle]
pub extern "C" fn int_from_rust() -> i32{
    2
}



#[no_mangle]
pub extern fn hello() {
    let hello = String::from("Rust 함수");
    println!("{}",hello);
}