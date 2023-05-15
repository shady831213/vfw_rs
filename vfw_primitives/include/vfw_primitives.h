#ifndef __VFW_PRIMITIVES_H__
#define __VFW_PRIMITIVES_H__
#include <stdarg.h>
#include <stdint.h>
#define NTH_ARG(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, N, ...) N
#define COUNT_VARGS(...) NTH_ARG(__VA_ARGS__, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1)
#define va_args_to_ptr(first_arg, rest_args, rest_args_len, va_len, first_type, rest_type) \
    {                                                                                      \
        va_list args;                                                                      \
        va_start(args, (va_len));                                                          \
        first_arg = (first_type)va_arg(args, first_type);                                  \
        for (unsigned int i = 0; i < (rest_args_len); i++)                                 \
        {                                                                                  \
            (rest_args)[i] = (rest_type)va_arg(args, rest_type);                        \
        }                                                                                  \
        va_end(args);                                                                      \
    }

#define io_write8(a, v) *((volatile uint8_t *)(a)) = (v)
#define io_read8(a) (*((volatile uint8_t *)(a)))
#define io_write16(a, v) *((volatile uint16_t *)(a)) = (v)
#define io_read16(a) (*((volatile uint16_t *)(a)))
#define io_write32(a, v) *((volatile uint32_t *)(a)) = (v)
#define io_read32(a) (*((volatile uint32_t *)(a)))
#define io_write64(a, v) *((volatile uint64_t *)(a)) = (v)
#define io_read64(a) (*((volatile uint64_t *)(a)))
#endif