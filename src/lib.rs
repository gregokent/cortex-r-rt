
//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]
extern crate cortex_r_rt_macros as macros;
extern crate r0;

use core::fmt;
use core::sync::atomic::{self, Ordering};

pub use macros::{entry, exception, pre_init};

#[export_name = "error: cortex-r-rt appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();


/// Returns a pointer to the start of the heap
///
/// The returned pointer is guaranteed to be 4-byte aligned.
#[inline]
pub fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __sheap: u32;
    }

    unsafe { &mut __sheap }
}


#[allow(dead_code)]
extern "C" {
    pub fn UndefinedEntry();
    pub fn SVCall();
    pub fn PrefetchAbort();
    pub fn DataAbort();
    pub fn PhantomInterrupt();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {

        // These symbols come from `link.x`
        static mut __sbss: u32;
        static mut __ebss: u32;

        static mut __sdata: u32;
        static mut __edata: u32;
        static __sidata: u32;

        pub fn coreInitRegisters();
        pub fn coreInitStackPointer();
    }

    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();
    }

    coreInitRegisters();
    coreInitStackPointer();

    __pre_init();

    // Initialize RAM
    r0::zero_bss(&mut __sbss, &mut __ebss);
    r0::init_data(&mut __sdata, &mut __edata, &__sidata);

    main()
}


#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler2_() -> ! {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        //atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() -> ! {
    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

/* Exceptions */
#[doc(hidden)]
pub enum Exception {
    UndefinedEntry,

    SVCall,

    PrefetchAbort,

    DataAbort,

    PhantomInterrupt,

}

// If we are not targeting a specific device we bind all the potential device specific interrupts
// to the default handler
#[cfg(all(not(feature = "device"), not(armv6m)))]
#[doc(hidden)]
#[link_section = ".vim_table"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 128] = [{
    extern "C" {
        fn DefaultHandler();
    }

    DefaultHandler
}; 128];


