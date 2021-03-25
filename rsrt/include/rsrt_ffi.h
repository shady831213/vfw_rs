#ifndef __RSRT_FFI_H__
#define __RSRT_FFI_H__
#include <stdint.h>
#include <stdarg.h>
#define NTH_ARG(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, N, ...) N
#define COUNT_VARGS(...) NTH_ARG(__VA_ARGS__, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1)
extern unsigned int hartid();
extern unsigned int save_flag();
extern void restore_flag(unsigned int flag);
extern void restore_flag(unsigned int flag);
extern void exit(int code);

extern void *malloc(unsigned int size, unsigned int align);
extern void free(void *ptr, unsigned int size, unsigned int align);

#define io_write32(a, v) *((volatile unsigned int *)(a)) = (v)
#define io_read32(a) (*((volatile unsigned int *)(a)))
#define va_args_to_ptr(first_arg, rest_args, rest_args_len, va_len, first_type, rest_type) \
    {                                                                                      \
        va_list args;                                                                      \
        va_start(args, va_len);                                                            \
        first_arg = (first_type)va_arg(args, first_type);                                  \
        for (unsigned int i = 0; i < args_len; i++)                                        \
        {                                                                                  \
            buf[i] = (unsigned int)va_arg(args, rest_type);                                \
        }                                                                                  \
        va_end(args);                                                                      \
    }

extern unsigned int c_fork(const void *entry, unsigned int args_len, unsigned int *args);
static inline unsigned int c_fork_wrapper(unsigned int args_len, ...)
{
    unsigned int buf[8];
    const void *entry;
    unsigned int num_args = (args_len - 1) > 8 ? 8 : args_len - 1;
    va_args_to_ptr(entry, buf, num_args, args_len, const void *, unsigned int);
    return c_fork(entry, num_args, buf);
}
#define fork(...) c_fork_wrapper(COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

extern unsigned int c_fork_on(unsigned int id, const void *entry, unsigned int args_len, unsigned int *args);
static inline unsigned int c_fork_on_wrapper(unsigned int id, unsigned int args_len, ...)
{
    unsigned int buf[8];
    const void *entry;
    unsigned int num_args = (args_len - 1) > 8 ? 8 : args_len - 1;
    va_args_to_ptr(entry, buf, num_args, args_len, const void *, unsigned int);
    return c_fork_on(id, entry, num_args, buf);
}
#define fork_on(id, ...) c_fork_on_wrapper(id, COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

extern int32_t c_try_fork(const void *entry, unsigned int args_len, unsigned int *args);
static inline int32_t c_try_fork_wrapper(unsigned int args_len, ...)
{
    unsigned int buf[8];
    const void *entry;
    unsigned int num_args = (args_len - 1) > 8 ? 8 : args_len - 1;
    va_args_to_ptr(entry, buf, num_args, args_len, const void *, unsigned int);
    return c_try_fork(entry, num_args, buf);
}
#define try_fork(...) c_try_fork_wrapper(COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

extern int32_t c_try_fork_on(unsigned int id, const void *entry, unsigned int args_len, unsigned int *args);
static inline int32_t c_try_fork_on_wrapper(unsigned int id, unsigned int args_len, ...)
{
    unsigned int buf[8];
    const void *entry;
    unsigned int num_args = (args_len - 1) > 8 ? 8 : args_len - 1;
    va_args_to_ptr(entry, buf, num_args, args_len, const void *, unsigned int);
    return c_try_fork_on(id, entry, num_args, buf);
}
#define try_fork_on(id, ...) c_try_fork_on_wrapper(id, COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

extern void join(unsigned int id);
#endif