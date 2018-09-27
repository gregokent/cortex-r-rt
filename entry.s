    .section .vector_table.exceptions,"a",%progbits
    .syntax unified
    .weak reset_entry

reset_entry:
    b   ResetTrampoline
    b   UndefinedEntryTrampoline
    b   SVCallTrampoline
    b   PrefetchAbortTrampoline
    b   DataAbortTrampoline
    b   PhantomInterruptTrampoline
    ldr pc,[pc,#-0x1b0]
    ldr pc,[pc,#-0x1b0]



    .section .text.ResetTrampoline
    .global ResetTrampoline
    .arm
    ResetTrampoline:
        b Reset


    .section .text.UndefinedEntryTrampoline
    .global UndefinedEntryTrampoline
    .arm
    UndefinedEntryTrampoline:
        b UndefinedEntry

    .section .text.SVCallTrampoline
    .global SVCallTrampoline
    .arm
    SVCallTrampoline:
        b SVCall

    .section .text.PrefetchAbortTrampoline
    .global PrefetchAbortTrampoline
    .arm
    PrefetchAbortTrampoline:
        b PrefetchAbort

    .section .text.DataAbortTrampoline
    .global DataAbortTrampoline
    .arm
    DataAbortTrampoline:
        b DataAbort

    .section .text.PhantomInterruptTrampoline
    .global PhantomInterruptTrampoline
    .arm
    PhantomInterruptTrampoline:
        b PhantomInterrupt
