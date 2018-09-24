
//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]
#![feature(global_asm, asm,naked_functions)]
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

#[naked]
pub unsafe fn __init_registers() {
    asm!("
        /* After reset, the CPU is in the Supervisor mode (M = 10011) */
        mov r0, lr
        mov r1, #0x0000
        mov r2, #0x0000
        mov r3, #0x0000
        mov r4, #0x0000
        mov r5, #0x0000
        mov r6, #0x0000
        mov r7, #0x0000
        mov r8, #0x0000
        mov r9, #0x0000
        mov r10, #0x0000
        mov r11, #0x0000
        mov r12, #0x0000
        mov r13, #0x0000
        mrs r1, cpsr
        msr spsr_cxsf, r1

        /* Switch to FIQ mode (M = 10001) */
        cps #17
        mov lr, r0
        mov r8, #0x0000
        mov r9, #0x0000
        mov r10, #0x0000
        mov r11, #0x0000
        mov r12, #0x0000
        mrs r1, cpsr
        msr spsr_cxsf, r1

        /* Switch to IRQ mode (M = 10010) */
        cps #18
        mov lr, r0
        mrs r1,cpsr
        msr spsr_cxsf, r1

        /* Switch to Abort mode (M = 10111) */
        cps #23
        mov lr, r0
        mrs r1,cpsr
        msr spsr_cxsf, r1

        /* Switch to Undefined Instruction Mode (M = 11011) */
        cps #27
        mov lr, r0
        mrs r1,cpsr
        msr spsr_cxsf, r1

        /* Switch to System Mode (Shares User Mode registers) (M = 11111) */
        cps #31
        mov lr, r0
        mrs r1,cpsr
        msr spsr_cxsf, r1

        mrc   p15,     #0x00,      r2,       c1, c0, #0x02
        orr   r2,      r2,         #0xF00000
        mcr   p15,     #0x00,      r2,       c1, c0, #0x02
        mov   r2,      #0x40000000
    ");

    #[cfg(vfp)]
    asm!("
        fmxr  fpexc,   r2
        fmdrr d0, r1, r1
        fmdrr d1, r1, r1
        fmdrr d2, r1, r1
        fmdrr d3, r1, r1
        fmdrr d4, r1, r1
        fmdrr d5, r1, r1
        fmdrr d6, r1, r1
        fmdrr d7, r1, r1
        fmdrr d8, r1, r1
        fmdrr d9, r1, r1
        fmdrr d10, r1, r1
        fmdrr d11, r1, r1
        fmdrr d12, r1, r1
        fmdrr d13, r1, r1
        fmdrr d14, r1, r1
        fmdrr d15, r1, r1
    ");

    asm!("
        bl    1f
    1:  bl    2f
    2:  bl    3f
    3:  bl    4f
    4:  bx r0
    ");
}

#[naked]
pub unsafe fn __init_stack_pointers() {
asm!("
    mov   r0, lr
    cps   #17
    ldr   sp,   fiq_sp
    cps   #18
    ldr   sp,   irq_sp
    cps   #19
    ldr   sp,   svc_sp
    cps   #23
    ldr   sp,   abort_sp
    cps   #27
    ldr   sp,   undef_sp
    cps   #31
    ldr   sp,   user_sp
    bx    r0

user_sp:  .word USER_SP
svc_sp:   .word SVC_SP
fiq_sp:   .word FIQ_SP
irq_sp:   .word IRQ_SP
abort_sp: .word ABORT_SP
undef_sp: .word UNDEF_SP
    ");
}

/* Entry point */
//#[doc(hidden)]
//#[link_section = ".vector_table.reset_vector"]
//#[no_mangle]
//pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
//
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

    }

    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();
    }

    __init_registers();
    __init_stack_pointers();

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

#[no_mangle]
extern "C" {
    pub fn UndefinedEntry();

    pub fn SVCall();

    pub fn PrefetchAbort();

    pub fn DataAbort();

    pub fn PhantomInterrupt();
}

global_asm!(r#"
    .cpu cortex-r4
    .arm

    .section .vector_table.exceptions,"a",%progbits
    .extern Reset
    .extern UndefinedEntry
    .extern SVCall
    .extern PrefetchAbort
    .extern DataAbort
    .extern DefaultHandler
    .weak reset_entry

reset_entry:
    ldr pc,ResetAddr
    ldr pc,UndefinedEntryAddr
    ldr pc,SVCallAddr
    ldr pc,PrefetchAbortAddr
    ldr pc,DataAbortAddr
    ldr pc,PhantomInterruptAddr
    ldr pc,[pc,#-0x1b0]
    ldr pc,[pc,#-0x1b0]

    ResetAddr:              .word Reset
    UndefinedEntryAddr:      .word UndefinedEntry
    SVCallAddr:              .word SVCall
    PrefetchAbortAddr:       .word PrefetchAbort
    DataAbortAddr:           .word DataAbort
    PhantomInterruptAddr:    .word PhantomInterrupt
"#);


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


