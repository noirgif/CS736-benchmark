extern crate libc;


fn main() {
    //pub unsafe extern fn getpid() -> pid_t

    unsafe {
        println!("{}", libc::getpid());
        println!("{}", libc::gettimeofday()");
    }
}