static mut WORK: AtomicBool = false;
pub fn new() -> Self {
       Self {
// callback function
//use WORK as
// substitute for local interrupt register
           signal:|_| unsafe {
                   Work = true;
               }
       }
   }
