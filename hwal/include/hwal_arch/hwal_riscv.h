#ifndef __HWAL_RISCV_H__
#define __HWAL_RISCV_H__
#include <stdint.h>
extern uint32_t mcycle();
extern void wait_mcycle(uint32_t cnt);

#define wait_util(cond, cnt) ({ \
    uint32_t cur = mcycle(); \
    uint32_t timeout = 1; \
    do { \
        if ((cond)){ \
            timeout = 0; \
            break; \
        } \
    } while((mcycle() - cur) < (cnt)); \
    timeout; \
})

#define read_csr(x) ({ \
    unsigned int r = 0; \
    __asm__ __volatile__ ("csrrs %0, %1, x0" : "=r" (r) : "i"((x))); \
    r; \
})

#define write_csr(x, v) { \
    __asm__ __volatile__ ("csrrw x0, %1, %0" : "r" ((v)) : "i"((x))); \
}

#define set_csr(x, v) { \
    __asm__ __volatile__ ("csrrs x0, %1, %0" : "r" ((v)) : "i"((x))); \
}

#define clr_csr(x, v) { \
    __asm__ __volatile__ ("csrrc x0, %1, %0" : "r" ((v)) : "i"((x))); \
}

typedef void* plic_t;

extern uint32_t plic_max_pri(plic_t plic);
extern void plic_set_pri(plic_t plic, uint32_t irq, uint32_t pri);
extern void plic_raise_int(plic_t plic, uint32_t irq);
extern void plic_set_threshold(plic_t plic, uint32_t threshold);
extern void plic_enable(plic_t plic, uint32_t irq);
extern void plic_disable(plic_t plic, uint32_t irq);
#endif