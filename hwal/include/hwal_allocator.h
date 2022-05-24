#ifndef __HWAL_ALLOCATOR_H__
#define __HWAL_ALLOCATOR_H__
#include <stdint.h>
typedef struct 
{
    uint32_t inner[8];
}allocator;

typedef struct 
{
    void* ptr;
    uint32_t size;
} al_block;

extern void allocator_init(allocator *a);
extern void allocator_add_block(allocator *a, uint32_t start, uint32_t size);
extern void *allocator_alloc(allocator *a, uint32_t size, uint32_t align);
extern void allocator_free(allocator *a, uint32_t start, uint32_t size);

#define al_new() (          \
    {                       \
        allocator a;        \
        allocator_init(&a); \
        a;                  \
    })
#define al_add_block(a, st, sz) allocator_add_block(a, (st), (sz))
#define al_alloc(a, sz, al) (                                          \
    {                                                                  \
        al_block block = {                                             \
            .ptr = allocator_alloc(a, (uint32_t)(sz), (uint32_t)(al)), \
            .size = (uint32_t)(sz)};                                   \
        block;                                                         \
    })
#define al_free(a, b) allocator_free(a, (uint32_t)((b).ptr), (uint32_t)((b).size))

#endif