    .section .intvecs,"a",%progbits
    .extern Reset
    .extern UndefinedEntry
    .extern SVCall
    .extern PrefetchAbort
    .extern DataAbort
    .extern DefaultHandler
    .weak reset_entry

reset_entry:
    b   Reset
    b   UndefinedEntry
    b   SVCall
    b   PrefetchAbort
    b   DataAbort
    b   DefaultHandler
    ldr pc,[pc,#-0x1b0]
    ldr pc,[pc,#-0x1b0]

