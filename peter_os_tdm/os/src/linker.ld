    stext = .;
    .text : {
        *(.text.entry)
        . = ALIGN(4k);
        strampoline = .;
        *(.text.trampoline);
        . = ALIGN(4k);
        *(.text .text.*)
    }