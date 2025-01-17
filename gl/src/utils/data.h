#include <stddef.h>
struct Vec {
    void * data;
    size_t size;
    size_t reserve;
    size_t item_size;
};

typedef struct Vec Vec;

Vec make_vec(size_t size, size_t item_size);
void release_vec(Vec* vec);
void vec_push(Vec* vec, void* item);
void* vec_extend(Vec* vec, size_t amount);
void vec_rem_swap(Vec* vec, size_t idx);
__attribute__((always_inline)) inline void* vec_i(Vec* vec, size_t idx) { return vec->data + idx * sizeof(vec->item_size); }
