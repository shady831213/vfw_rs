
SECTIONS
{    

    .init : ALIGN(4) {
        /* Place init sections first */
        KEEP(*(.init));
        KEEP(*(.init.*));
    } > REGION_INIT

    .got : ALIGN(4) {
        _sgot = .;
        *(.got .got.*);
        _egot = .;
    } > REGION_GOT

    .text : ALIGN(4) {
        _stext = .;
        *(.text .text.*)
        _etext = .;
    } > REGION_TEXT AT>REGION_TEXT_LOAD
    PROVIDE(_stext_load = LOADADDR(.text));

    .stack (NOLOAD) : ALIGN(1K) {
        _estack = .;
        . += {{stack_size}};
        _sstack = .;
    } > REGION_STACK

    .rodata : ALIGN(4) {
        _srodata = .;
        *(.srodata .srodata.*);
        *(.rodata .rodata.*)
        /* 4-byte align the end (VMA) of this section.
        This is required by LLD to ensure the LMA of the following .data
        section will have the correct alignment. */
        . = ALIGN(4);
        _erodata = .;
    } > REGION_RODATA AT>REGION_RODATA_LOAD
    PROVIDE(_srodata_load = LOADADDR(.rodata));


    .init.bss : {
        _s_init_bss = .;
        *(.rel_lottery)
        _e_init_bss = .;
    } > REGION_INIT_BSS

    .bss {{load_bss}} : ALIGN(4) {
        _sbss = .;
        *(.bss .bss.* .sbss .sbss.*)
        _ebss = .;
    } > REGION_BSS

    .cpu.bss {{load_bss}} : ALIGN(4) {
        _s_cpu_bss = .;
        *(.cpu.bss .cpu.bss.*)
        _e_cpu_bss = .;
    } > REGION_CPU_BSS

    .synced.bss {{load_bss}} : ALIGN(4) {
        _s_synced_bss = .;
        *(.synced.bss .synced.bss.*)
        _e_synced_bss = .;
    } > REGION_SYNCED_BSS

    .data : ALIGN(4) {
        _sdata = .;
        /* Must be called __global_pointer$ for linker relaxations to work. */
        PROVIDE(__global_pointer$ = . + 0x800);
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*)
        _edata = .;
    } > REGION_DATA AT>REGION_DATA_LOAD
    PROVIDE(_sdata_load = LOADADDR(.data));

    .synced.data : ALIGN(4) {
        _s_synced_data = .;
        *(.synced.data .synced.data.*)
        _e_synced_data = .;
    } > REGION_SYNCED_DATA AT>REGION_SYNCED_DATA_LOAD
    PROVIDE(_s_synced_data_load = LOADADDR(.synced.data));

    .cpu.data : ALIGN(4) {
        _s_cpu_data = .;
        *(.cpu.data .cpu.data.*)
        _e_cpu_data = .;
    } > REGION_CPU_DATA AT>REGION_CPU_DATA_LOAD
    PROVIDE(_s_cpu_data_load = LOADADDR(.cpu.data));

    .heap (NOLOAD) : ALIGN(4) {
        _sheap = .;
        . += _heap_size;
        . = ALIGN(4);
        _eheap = .;
    } > REGION_HEAP
}

ASSERT((_stack_size) % 1K == 0, "
ERROR: stack_size must be align with 1K.");

ASSERT((_num_cores) > 0, "
ERROR: _num_cores must be at least 1.");