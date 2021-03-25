MEMORY
{
  DLM : ORIGIN = 4096, LENGTH = 4K
  GLOBAL: ORIGIN = 0x80000000, LENGTH = 1M
}

OUTPUT_ARCH(riscv)
ENTRY(_start)

REGION_ALIAS("REGION_MAILBOX", DLM);
REGION_ALIAS("REGION_STACK", DLM);
REGION_ALIAS("REGION_TEXT", GLOBAL);
REGION_ALIAS("REGION_RODATA", GLOBAL);
REGION_ALIAS("REGION_DATA", GLOBAL);
REGION_ALIAS("REGION_HEAP", GLOBAL);
REGION_ALIAS("REGION_BSS", GLOBAL);

PROVIDE(_stack_size = 3K);
PROVIDE(_heap_size = 4k);
PROVIDE(_num_cores = 3);
