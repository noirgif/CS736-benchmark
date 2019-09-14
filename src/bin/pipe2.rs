extern crate libc;


fn main(){
    unsafe {
        libc::printf(&'static str"hello");
    }
}