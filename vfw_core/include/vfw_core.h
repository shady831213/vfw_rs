#ifndef __VFW_CORE_H__
#define __VFW_CORE_H__
#include <stdint.h>
#include <vfw_primitives.h>
extern uintptr_t num_cores();
extern uintptr_t hartid();
extern uintptr_t save_flag();
extern void restore_flag(uintptr_t flag);
extern void mem_wb(uintptr_t start, uintptr_t size);
extern void mem_flush(uintptr_t start, uintptr_t size);
extern void mem_invalid(uintptr_t start, uintptr_t size);
extern void exit(int code);


extern uint32_t c_fork(const void *entry, uintptr_t args_len, uintptr_t *args);
static inline uint32_t c_fork_wrapper(uintptr_t args_len, ...)
{
    uintptr_t buf[8];
    const void *entry;
    uintptr_t num_args = (args_len - 1) > 8 ? 8 : args_len - 1;
    va_args_to_ptr(entry, buf, num_args, args_len, const void *, uintptr_t);
    return c_fork(entry, num_args, buf);
}
#define fork(...) c_fork_wrapper(COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

extern uint32_t c_fork_on(uintptr_t id, const void *entry, uintptr_t args_len, uintptr_t *args);
static inline uintptr_t c_fork_on_wrapper(uintptr_t id, uintptr_t args_len, ...)
{
    uintptr_t buf[8];
    const void *entry;
    uintptr_t num_args = (args_len - 1) > 8 ? 8 : args_len - 1;
    va_args_to_ptr(entry, buf, num_args, args_len, const void *, uintptr_t);
    return c_fork_on(id, entry, num_args, buf);
}
#define fork_on(id, ...) c_fork_on_wrapper(id, COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

extern int32_t c_try_fork(const void *entry, uintptr_t args_len, uintptr_t *args);
static inline intptr_t c_try_fork_wrapper(uintptr_t args_len, ...)
{
    uintptr_t buf[8];
    const void *entry;
    uintptr_t num_args = (args_len - 1) > 8 ? 8 : args_len - 1;
    va_args_to_ptr(entry, buf, num_args, args_len, const void *, uintptr_t);
    return c_try_fork(entry, num_args, buf);
}
#define try_fork(...) c_try_fork_wrapper(COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

extern int32_t c_try_fork_on(uintptr_t id, const void *entry, uintptr_t args_len, uintptr_t *args);
static inline intptr_t c_try_fork_on_wrapper(uintptr_t id, uintptr_t args_len, ...)
{
    uintptr_t buf[8];
    const void *entry;
    uintptr_t num_args = (args_len - 1) > 8 ? 8 : args_len - 1;
    va_args_to_ptr(entry, buf, num_args, args_len, const void *, uintptr_t);
    return c_try_fork_on(id, entry, num_args, buf);
}
#define try_fork_on(id, ...) c_try_fork_on_wrapper(id, COUNT_VARGS(__VA_ARGS__), __VA_ARGS__)

extern void join(uint32_t id);
#endif