#ifndef PARC_CORPUS_INCLUDE_E_API_H
#define PARC_CORPUS_INCLUDE_E_API_H

#if !defined(INCLUDE_CALL)
#  error "platform/order.h must be included before surface/api.h"
#endif

typedef struct include_frame {
    include_counter_t INCLUDE_FIELD(sequence);
    size_t INCLUDE_FIELD(length);
    include_log_sink INCLUDE_FIELD(log_sink);
} include_frame;

INCLUDE_API include_handle *INCLUDE_CALL include_open(const include_frame *frame);
INCLUDE_API int INCLUDE_CALL include_flush(include_handle *handle, include_log_sink sink);

#endif
