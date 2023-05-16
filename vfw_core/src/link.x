
SECTIONS
{    
    .text : {
        _stext = .;
        /* Place init sections first */
        KEEP(*(.init));
        KEEP(*(.init.rust));
        *(.text .text.*)
        _etext = .;
    } > REGION_TEXT

    .stack (NOLOAD) : ALIGN(1K) {
        _estack = .;
        . += {{stack_size}};
        _sstack = .;
    } > REGION_STACK

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

    .bss : {
        _sbss = .;
        *(.bss .bss.* .sbss .sbss.*)
        _ebss = .;
    } > REGION_BSS

    .cpu.bss : {
        _s_cpu_bss = .;
        *(.cpu.bss .cpu.bss.*)
        _e_cpu_bss = .;
    } > REGION_CPU_BSS

    .synced.bss : {
        _s_synced_bss = .;
        *(.synced.bss .synced.bss.*)
        _e_synced_bss= .;
    } > REGION_SYNCED_BSS

    .data : {
        sidata = LOADADDR(.data);
        _sdata = .;
        /* Must be called __global_pointer$ for linker relaxations to work. */
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*)
        _edata = .;
    } > REGION_DATA

    .synced.data : {
        sisynceddata = LOADADDR(.synced.data);
        _s_synced_data = .;
        *(.synced.data .synced.data.*)
        _e_synced_data = .;
    } > REGION_SYNCED_DATA

    .cpu.data : {
        sicpudata = LOADADDR(.cpu.data);
        _s_cpu_data = .;
        *(.cpu.data .cpu.data.*)
        _e_cpu_data = .;
    } > REGION_CPU_DATA

    .heap (NOLOAD) : {
        _sheap = .;
        . += _heap_size - _provide_base;
        . = ALIGN(4);
        _eheap = .;
    } > REGION_HEAP
}

/* _provide_base is for R_RISCV_PCREL_HI20 issue like https://github.com/rust-embedded/riscv-rt/issues/107  */

ASSERT((_stack_size - _provide_base) % 1K == 0, "
ERROR: stack_size must be align with 1K.");

ASSERT((_num_cores - _provide_base) > 0, "
ERROR: _num_cores must be at least 1.");