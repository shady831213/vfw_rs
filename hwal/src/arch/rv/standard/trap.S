.section .text, "ax"
.global _vector_table
.align 2
_vector_table:
    j _start_trap
    j _start_trap
    j _start_trap
    j _start_trap
    j _start_trap
    j _start_trap
    j _start_trap
    j _start_trap
    j _start_trap
    j _start_trap
    j _start_trap
    j _start_trap                