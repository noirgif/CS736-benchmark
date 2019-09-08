use std::time::{Duration, Instant};
use std::thread::sleep;
extern crate core;
use core::arch::x86_64::__rdtscp;
// extern "C" {
//     pub unsafe fn __rdtscp(aux: *mut u32) -> u64
// }

fn main() {
   let now = Instant::now();

let mut z = 0;

//   // we sleep for 2 seconds
//   for x in 1..2 {
//       z = z + x;
//   }
   
   let m = now.elapsed().as_nanos();
   println!("{} {}", m, z);

   let mut aux : u32 = 0;
   let mut t1 = 0u64;
   let mut t2 = 0u64;

   unsafe {
      t1 =  __rdtscp(&mut aux);

   }

   //for i in 1..5 {

//   }

unsafe {
      t2 =__rdtscp(&mut aux);
   }
println!("diff = {}", t2-t1);


}