use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct RustObject {
    Key: i32,
    dDiameter : f64,
    enType : i32,
    // Other members...
}

#[repr(C)]
pub struct  RustRebarSpecInfo {
	strName: *mut c_char,
	dDiameter: f64,
	dBendingRadius: f64,
	dHookLength_90: f64,
	dHookLength_135: f64,
	dHookLength_180: f64,
	dWeight: f64
}

#[repr(C)]
pub struct RebarSpecification_fromRust {
    Key: i32,
    strNamme: *mut c_char,
    specLength: i32,
    ptrSpecInfo: *mut RustRebarSpecInfo,
}

#[no_mangle]
pub extern "C" fn struct_from_rust() -> RustObject{
    RustObject{Key : 3, dDiameter : 5.0, enType : 2}
}

#[no_mangle]
pub extern "C" fn GetRebarSpecification() -> RebarSpecification_fromRust{
    let mut name = String::from("KS D 3504");
    let c_str_name = CString::new(name).unwrap();

    let mut vec = Vec::new();

    let mut firstSpecName = String::from("D4");
    let c_str_firstSpecName = CString::new(firstSpecName).unwrap();

    vec.push(
        RustRebarSpecInfo{
            strName : c_str_firstSpecName.into_raw(),  
            dDiameter : 0.0042,
            dBendingRadius : 0.0150,
            dHookLength_90 : 0.0510,
            dHookLength_135 : 0.03,
            dHookLength_180 : 0.06,
            dWeight : 1.08
        });

        let mut secondSpecName = String::from("D5");
        let c_str_secondSpecName = CString::new(secondSpecName).unwrap();
        vec.push(
            RustRebarSpecInfo{
                strName : c_str_secondSpecName.into_raw(),  
                dDiameter : 0.0053,
                dBendingRadius : 0.0190,
                dHookLength_90 : 0.0640,
                dHookLength_135 : 0.03,
                dHookLength_180 : 0.06,
                dWeight : 1.70
            });

    let boxed_slice: Box<[RustRebarSpecInfo]> = vec.into_boxed_slice();
    let len = boxed_slice.len();
    let fat_ptr: *mut [RustRebarSpecInfo] =
        Box::into_raw(boxed_slice)
    ;
    let slim_ptr: *mut RustRebarSpecInfo = fat_ptr as _;
    RebarSpecification_fromRust{Key : 1, strNamme : c_str_name.into_raw(), specLength : len as i32, ptrSpecInfo : slim_ptr}
}

#[no_mangle]
pub extern "C" fn theme_song_generate() -> *mut c_char {
    let mut song = String::from("1");
    song.push_str("2");

    let c_str_song = CString::new(song).unwrap();
    c_str_song.into_raw()
}
