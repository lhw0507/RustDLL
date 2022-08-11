use std::os::raw::c_char;

#[repr(C)]
pub struct  RustRebarSpecInfo {
	pub strName: *mut c_char,
	pub dDiameter: f64,
	pub dBendingRadius: f64,
	pub dHookLength_90: f64,
	pub dHookLength_135: f64,
	pub dHookLength_180: f64,
	pub dWeight: f64
}

#[repr(C)]
pub struct RebarSpecification_fromRust {
    pub Key: i32,
    pub strNamme: *mut c_char,
    pub specLength: i32,
    pub ptrSpecInfo: *mut RustRebarSpecInfo,
}
