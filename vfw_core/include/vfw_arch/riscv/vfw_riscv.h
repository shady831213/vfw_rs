#ifndef __VFW_RISCV_H__
#define __VFW_RISCV_H__
#include <stdint.h>
#include <assert.h>
#include <vfw_core.h>
extern uintptr_t mcycle();
extern void wait_mcycle(uintptr_t cnt);

#define wait_util(cond, cnt) ({ \
    uintptr_t cur = mcycle(); \
    uintptr_t timeout = 1; \
    do { \
        if ((cond)){ \
            timeout = 0; \
            break; \
        } \
    } while((mcycle() - cur) < (cnt)); \
    timeout; \
})

extern uint64_t mcycle64();
extern void wait_mcycle64(uint64_t cnt);


#define wait_util64(cond, cnt) ({ \
    uint64_t cur = mcycle64(); \
    uint64_t timeout = 1; \
    do { \
        if ((cond)){ \
            timeout = 0; \
            break; \
        } \
    } while((mcycle64() - cur) < (cnt)); \
    timeout; \
})

#define read_csr(x) ({ \
    unsigned int r = 0; \
    __asm__ volatile ("csrrs %0, %1, x0" : "=r" (r) : "i"((x))); \
    r; \
})

#define write_csr(x, v) ({ \
    unsigned int r = (v); \
    __asm__ volatile ("csrrw x0, %1, %0" : "+r" (r) : "i"((x))); \
})

#define set_csr(x, v) { \
    unsigned int r = (v); \
    __asm__ volatile ("csrrs x0, %1, %0" : "+r" (r) : "i"((x))); \
}

#define clr_csr(x, v) { \
    unsigned int r = (v); \
    __asm__ volatile ("csrrc x0, %1, %0" : "+r" (r) : "i"((x))); \
}

#define get_sp() ({ \
    register unsigned int sp; \
    __asm__ volatile ("mv %0, sp" : "=r" (sp)); \
    sp; \
})

#define check_stack() ({ \
    unsigned int start = stack_start(); \
    unsigned int end = start - stack_size(); \
    assert((get_sp() <= start) && (get_sp() > end)); \
})

#endif