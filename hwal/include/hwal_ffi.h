#ifndef __HWAL_FFI_H__
#define __HWAL_FFI_H__
#include <rsrt_ffi.h>
extern void cprint(const char *fmt, const char *file, unsigned int line, unsigned int arg_len, unsigned int *args);
static inline void printf_wrapper(const char *file, unsigned int line, unsigned int args_len, ...)
{
    unsigned int buf[16];
    const char *fmt;
    unsigned int num_args = (args_len - 1) > 16 ? 16 : args_len - 1;
    va_args_to_ptr(fmt, buf, num_args, args_len, const char *, unsigned int);
    cprint(fmt, file, line, num_args, buf);
}
#define printf(...) printf_wrapper(__FILE__, __LINE__, COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)
#define float_to_arg(f) ({                  \
    float _f = (f);                         \
    (unsigned int)(*(unsigned int *)(&_f)); \
})
#endif