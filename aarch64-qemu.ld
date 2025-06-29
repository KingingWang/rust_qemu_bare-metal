ENTRY(_start)

MEMORY {
    RAM (rwx) : ORIGIN = 0x40080000, LENGTH = 1024M
}

SECTIONS {
    .text.boot : {
        *(.text.boot)
    } > RAM

    .text : {
        *(.text .text.*)
    } > RAM

    .rodata : ALIGN(8) {
        *(.rodata .rodata.* .rodata.cst8 .rodata..Lanon.*) 
    } > RAM

    .data : ALIGN(8) { 
        *(.data .data.*)
    } > RAM

    .bss : ALIGN(8) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } > RAM

    .stack (NOLOAD) : ALIGN(16) {
        . = . + 0x40000;
        LD_STACK_PTR = .;
    } > RAM
}
