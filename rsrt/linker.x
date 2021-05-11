
SECTIONS
{
    .stack (NOLOAD) : ALIGN(1K) {
        _estack = .;
        . += _stack_size;
        _sstack = .;
    } > REGION_STACK
    
    .text : {
        _stext = .;
        /* Place init sections first */
        KEEP(*(.init));
        KEEP(*(.init.rust));
        *(.text .text.*)
        _etext = .;
    } > REGION_TEXT

    .rodata : {
        _srodata = .;
        *(.srodata .srodata.*);
        *(.rodata .rodata.*)
        /* 4-byte align the end (VMA) of this section.
        This is required by LLD to ensure the LMA of the following .data
        section will have the correct alignment. */
        . = ALIGN(4);
        _erodata = .;
    } > REGION_RODATA

    .data : {
        sidata = LOADADDR(.data);
        _sdata = .;
        /* Must be called __global_pointer$ for linker relaxations to work. */
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*)
        _edata = .;
    } > REGION_DATA

    .bss : {
        _sbss = .;
        *(.bss .bss.* .sbss .sbss.*)
        _ebss = .;
    } > REGION_BSS

    .heap (NOLOAD) : {
        _sheap = .;
        . += _heap_size;
        . = ALIGN(4);
        _eheap = .;
    } > REGION_HEAP

    /DISCARD/ : {
        *(.eh_frame .eh_frame_hdr,.riscv.attributes, .debug_*, .comment);
    }
}
