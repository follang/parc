#ifndef PARC_CORPUS_INCLUDE_E_TYPES_H
#define PARC_CORPUS_INCLUDE_E_TYPES_H

#if !defined(INCLUDE_API)
#  error "platform/order.h must be included before platform/types.h"
#endif

#include <stddef.h>
#include <stdint.h>
#include <stdarg.h>

#if __SIZEOF_LONG__ >= 8
typedef uint64_t include_counter_t;
#else
typedef uint32_t include_counter_t;
#endif

typedef struct include_handle include_handle;
typedef int (*include_log_sink)(include_handle *handle, const char *fmt, va_list ap);

#endif
