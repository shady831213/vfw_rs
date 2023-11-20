#ifndef __VFW_CLINT_H__
#define __VFW_CLINT_H__
#include <stdint.h>
typedef void* clint_t;

extern uint64_t clint_get_mtime(clint_t clint);
extern void clint_set_timer(clint_t clint, uint64_t instant);
#endif