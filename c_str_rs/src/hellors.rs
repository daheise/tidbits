#![feature(libc)]
extern crate libc;
use std::str;
use std::ptr;
use std::ffi::CStr;

#[link(name = "hello")]
extern{
    fn hello() -> *mut libc::c_char;
}

fn hi() -> String {
    unsafe{
        let cstr = hello();
        let len = libc::strlen(cstr) as usize;
        let mut str_data:Vec<u8> = Vec::with_capacity(len);
        str_data.set_len(len);
        ptr::copy(cstr as *const u8, str_data.as_mut_ptr(), len);
        libc::free(cstr as *mut libc::c_void);
        return String::from_utf8(str_data).unwrap();
    }
}

fn hi2() -> String{
    let tmp = unsafe { CStr::from_ptr(hello()) };
    let retval = str::from_utf8(tmp.to_bytes()).unwrap_or("").to_owned();
    unsafe {libc::free(tmp.as_ptr() as *mut libc::c_void);}
    return retval;
}

fn main(){
  println!("{}", hi());
}

#[test]
fn test_say_hi(){
    assert!(hi() == "Hello");
}

#[test]
fn test_say_hi2(){
    assert!(hi2() == "Hello");
}
