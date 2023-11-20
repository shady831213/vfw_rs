#ifndef __VFW_PLIC_H__
#define __VFW_PLIC_H__
#include <stdint.h>
typedef void* plic_t;

extern uint32_t plic_max_pri(plic_t plic);
extern void plic_set_pri(plic_t plic, uint32_t irq, uint32_t pri);
extern void plic_raise_int(plic_t plic, uint32_t irq);
extern void plic_set_threshold(plic_t plic, uint32_t threshold);
extern void plic_enable(plic_t plic, uint32_t irq);
extern void plic_disable(plic_t plic, uint32_t irq);
#endif