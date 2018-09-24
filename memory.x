/* Device specific memory layout */

/* This file is used to build the cortex-m-rt examples,
   but not other applications using cortex-m-rt. */

MEMORY
{
  /* FLASH and RAM are mandatory memory regions */
  /* Update examples/data_overflow.rs if you change these sizes. */
  FLASH : ORIGIN = 0x00000000, LENGTH = 4M
  RAM : ORIGIN = 0x08001500, LENGTH = 0x3EB00
  VIMRAM : ORIGIN = 0xFFF82000, LENGTH = 0x4A0
  /* More memory regions can declared: for example this is a second RAM region */
  /* CCRAM : ORIGIN = 0x10000000, LENGTH = 8K */
}

STACK_ORIGIN = 0x08000000;
STACK_LENGTH = 0x00001500;
RAM_ORIGIN   = 0x08001500;
RAM_LENGTH   = 0x0003EB00;
VIM_ORIGIN   = 0xFFF82000;
VIM_LENGTH   = 0x000004A0;

HEAP_LENGTH  = 0x00001500;

MPU_MIN_ALIGN = 8K;

/* Stack pointers in all CPU modes */
FIQ_SP   = 0x08001200;
IRQ_SP   = 0x08001300;
SVC_SP   = 0x08001100;
ABORT_SP = 0x08001400;
UNDEF_SP = 0x08001500;
USER_SP  = 0x08001000;

/* The location of the stack can be overridden using the `_stack_start` symbol.
   By default it will be placed at the end of the RAM region */
/* _stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM); */

/* The location of the .text section can be overridden using the `_stext` symbol.
   By default it will place after .vector_table */
/* _stext = ORIGIN(FLASH) + 0x40c; */
