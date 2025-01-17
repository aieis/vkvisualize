#include "data.h"

#include <stdlib.h>
#include <string.h>

Vec make_vec(size_t reserve, size_t item_size) {
    Vec vec;
    vec.size = 0;
    vec.reserve = reserve;
    vec.item_size = item_size;
    vec.data = malloc(vec.item_size * vec.reserve);
    return vec;        
}

void release_vec(Vec* vec) {
    free(vec->data);
}

void vec_push(Vec* vec, void* item) {
    if (vec->size == vec->reserve) {
        vec->reserve = (vec->reserve + 1) * 1.5;
        vec->data = realloc(vec->data, vec->item_size * vec->reserve);        
    }

    memcpy(vec->data + vec->size * sizeof(vec->item_size), item, vec->item_size);
    vec->size++;
}

void* vec_extend(Vec* vec, size_t amount) {
    vec->reserve += amount;
    vec->data = realloc(vec->data, vec->item_size * vec->reserve);
    return vec->data + vec->size * sizeof(vec->item_size);
}

void vec_rem_swap(Vec* vec, size_t idx) {
    memcpy(vec->data + idx * sizeof(vec->item_size), vec->data + vec->size * sizeof(vec->item_size), vec->item_size);
    vec->size--;
}
