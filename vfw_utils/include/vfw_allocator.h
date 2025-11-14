#ifndef __VFW_ALLOCATOR_H__
#define __VFW_ALLOCATOR_H__
#include <stdint.h>
typedef struct
{
    uintptr_t inner[8];
} allocator;

typedef struct
{
    void *ptr;
    uintptr_t size;
} al_block;

extern void allocator_init(allocator *a);
extern void allocator_add_block(allocator *a, uintptr_t start, uintptr_t size);
extern void *allocator_alloc(allocator *a, uintptr_t size, uintptr_t align);
extern void allocator_free(allocator *a, uintptr_t start, uintptr_t size);

#define al_new() (           \
    {                        \
        allocator a = {{0}}; \
        allocator_init(&a);  \
        a;                   \
    })
#define al_add_block(a, st, sz) allocator_add_block(a, (st), (sz))
#define al_alloc(a, sz, al) (                                            \
    {                                                                    \
        al_block block = {                                               \
            .ptr = allocator_alloc(a, (uintptr_t)(sz), (uintptr_t)(al)), \
            .size = (uintptr_t)(sz)};                                    \
        block;                                                           \
    })
#define al_free(a, b) allocator_free(a, (uintptr_t)((b).ptr), (uintptr_t)((b).size))

#define al_reset(a) allocator_init(a)

#endif