#ifndef __VFW_MB_H__
#define __VFW_MB_H__
#include <stdint.h>
#include <vfw_primitives.h>
extern void mailbox_cprint(const char *fmt, const char *file, unsigned int line, unsigned int arg_len, uintptr_t *args);
static inline void printf_wrapper(const char *file, unsigned int line, unsigned int args_len, ...)
{
    uintptr_t buf[16];
    const char *fmt;
    unsigned int num_args = (args_len - 1) > 16 ? 16 : args_len - 1;
    va_args_to_ptr(fmt, buf, num_args, args_len, const char *, uintptr_t);
    mailbox_cprint(fmt, file, line, num_args, buf);
}
#define printf(...) printf_wrapper(__FILE__, __LINE__, COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)
#define float_to_arg(f) ({                  \
    float _f = (f);                         \
    (uintptr_t)(*(uint32_t *)(&_f)); \
})

extern unsigned int mailbox_call(const char *method, unsigned int arg_len, uintptr_t *args);
static inline unsigned int mbcall_wrapper(unsigned int args_len, ...)
{
    uintptr_t buf[16];
    const char *method;
    unsigned int num_args = (args_len - 1) > 16 ? 16 : args_len - 1;
    va_args_to_ptr(method, buf, num_args, args_len, const char *, uintptr_t);
    return mailbox_call(method, num_args, buf);
}
#define svcall(...) mbcall_wrapper(COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)
#define mbcall(...) mbcall_wrapper(COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

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

#define fread(fd, buf, len) mailbox_fread(fd, (void *)(buf), len)

#define fwrite(fd, buf, len) mailbox_fwrite(fd, (void *)(buf), len)

static inline unsigned int fseek(FD fd, unsigned int pos)
{
    return mailbox_fseek(fd, pos);
}


extern void* mailbox_memmove(void *dest, const void *src, unsigned int size);
extern void* mailbox_memset(void *dest, int data, unsigned int size);
extern int mailbox_memcmp(const void *s1, const void *s2, unsigned int size);

#define bd_memmove(dest, src, size) mailbox_memmove((void *)(dest), (void *)(src), (unsigned int)(size))

#define bd_memcpy(dest, src, size) mailbox_memmove((void *)(dest), (void *)(src), (unsigned int)(size))

#define bd_memset(dest, data, size) mailbox_memset((void *)(dest), (unsigned int)((unsigned char)(data)), (unsigned int)(size))

#define bd_memcmp(s1, s2, size) mailbox_memcmp((void *)(s1), (void *)(s2), (unsigned int)(size))

#endif