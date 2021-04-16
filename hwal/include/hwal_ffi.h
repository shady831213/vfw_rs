#ifndef __HWAL_FFI_H__
#define __HWAL_FFI_H__
#include <rsrt_ffi.h>
extern void mailbox_cprint(const char *fmt, const char *file, unsigned int line, unsigned int arg_len, unsigned int *args);
static inline void printf_wrapper(const char *file, unsigned int line, unsigned int args_len, ...)
{
    unsigned int buf[16];
    const char *fmt;
    unsigned int num_args = (args_len - 1) > 16 ? 16 : args_len - 1;
    va_args_to_ptr(fmt, buf, num_args, args_len, const char *, unsigned int);
    mailbox_cprint(fmt, file, line, num_args, buf);
}
#define printf(...) printf_wrapper(__FILE__, __LINE__, COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)
#define float_to_arg(f) ({                  \
    float _f = (f);                         \
    (unsigned int)(*(unsigned int *)(&_f)); \
})

extern unsigned int mailbox_svcall(const char *method, unsigned int arg_len, unsigned int *args);
static inline unsigned int svcall_wrapper(unsigned int args_len, ...)
{
    unsigned int buf[15];
    const char *method;
    unsigned int num_args = (args_len - 1) > 15 ? 15 : args_len - 1;
    va_args_to_ptr(method, buf, num_args, args_len, const char *, unsigned int);
    return mailbox_svcall(method, num_args, buf);
}
#define svcall(...) svcall_wrapper(COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

#define MB_FILE_READ 0x1
#define MB_FILE_WRITE 0x2
#define MB_FILE_APPEND 0x4
#define MB_FILE_TRUNC 0x8

#ifndef FD
#define FD unsigned int
#endif

#ifndef FILE_READ
#define FILE_READ MB_FILE_READ
#endif

#ifndef FILE_WRITE
#define FILE_WRITE MB_FILE_WRITE
#endif

#ifndef FILE_APPEND
#define FILE_APPEND MB_FILE_APPEND
#endif

#ifndef FILE_TRUNC
#define FILE_TRUNC MB_FILE_TRUNC
#endif

extern unsigned int mailbox_fopen(const char *path, unsigned int flags);
extern void mailbox_fclose(unsigned int fd);
extern unsigned int mailbox_fread(unsigned int fd, void *buf, unsigned int len);
extern unsigned int mailbox_fwrite(unsigned int fd, const void *buf, unsigned int len);
extern unsigned int mailbox_fseek(unsigned int fd, unsigned int pos);

static inline FD fopen(const char *path, unsigned int flags)
{
    return mailbox_fopen(path, flags);
}

static inline void fclose(FD fd)
{
    mailbox_fclose(fd);
}

static inline unsigned int fread(FD fd, void *buf, unsigned int len)
{
    return mailbox_fread(fd, buf, len);
}

static inline unsigned int fwrite(FD fd, const void *buf, unsigned int len)
{
    return mailbox_fwrite(fd, buf, len);
}

static inline unsigned int fseek(FD fd, unsigned int pos)
{
    return mailbox_fseek(fd, pos);
}

#endif