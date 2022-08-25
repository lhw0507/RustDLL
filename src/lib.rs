extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn GetRebarSpecification() -> *mut c_char{
    let json = r#"
    {
        "CRebarSpecification": {
          "Key": 1,
          "strName": "KS D 3504",
          "taRebarSpecification": [
            {
              "strName" : "D4",  
              "dDiameter" : 0.0042,
              "dBendingRadius" : 0.0150,
              "dHookLength_90" : 0.0510,
              "dHookLength_135" : 0.03,
              "dHookLength_180" : 0.06,
              "dWeight" : 1.08
            },
            {
              "strName" : "D5",  
              "dDiameter" : 0.0053,
              "dBendingRadius" : 0.0190,
              "dHookLength_90" : 0.0640,
              "dHookLength_135" : 0.03,
              "dHookLength_180" : 0.06,
              "dWeight" : 1.70
            }
          ]
        }
      }
    "#;
    let c_str_json = CString::new(json).unwrap();
    c_str_json.into_raw()
}

#[no_mangle]
pub extern "C" fn string_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn CreateRebarSpecification(input : *const c_char) -> bool {
    //do something
    true
}
