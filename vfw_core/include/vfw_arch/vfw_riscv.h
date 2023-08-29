#ifndef __VFW_RISCV_H__
#define __VFW_RISCV_H__
#include <stdint.h>
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

typedef void* plic_t;

extern uint32_t plic_max_pri(plic_t plic);
extern void plic_set_pri(plic_t plic, uint32_t irq, uint32_t pri);
extern void plic_raise_int(plic_t plic, uint32_t irq);
extern void plic_set_threshold(plic_t plic, uint32_t threshold);
extern void plic_enable(plic_t plic, uint32_t irq);
extern void plic_disable(plic_t plic, uint32_t irq);
#endif