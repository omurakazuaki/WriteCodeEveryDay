#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define bool	_Bool

typedef struct {
  void *raw;
  size_t capacity;
  size_t size;
  size_t item_size;
} vector;

vector v_new(size_t size) {
  vector vec;
  vec.capacity = 8;
  vec.item_size = size;
  vec.raw = (void*)calloc(vec.capacity, vec.item_size);
  vec.size = 0;
  return vec;
}

bool is_empty(vector* vec) {
  return vec->size == 0;
}

void v_resize(vector* vec) {
  if (vec->capacity == vec->size) {
    vec->capacity = vec->capacity << 1;
    vec->raw = realloc(vec->raw, vec->capacity * vec->item_size);
    memset(vec->raw + (vec->size * vec->item_size), 0, vec->capacity >> 1 * vec->item_size);
  }
}

void v_push(vector* vec, void *val) {
  v_resize(vec);
  void *p = vec->raw + (vec->size * vec->item_size);
  memcpy(p, val, vec->item_size);
  vec->size += 1;
}

void v_insert(vector* vec, int index, void *val) {
  v_resize(vec);
  void *p = vec->raw + (index * vec->item_size);
  memcpy(p + vec->item_size, p, (vec->size - index) * vec->item_size);
  memcpy(p, val, vec->item_size);
  vec->size += 1;
}

void v_delete(vector* vec, int index) {
  void *p = vec->raw + (index * vec->item_size);
  memcpy(p, p + vec->item_size, (vec->size - index) * vec->item_size);
  vec->size -= 1;
}

void* v_get(vector* vec, int index) {
  void *p = vec->raw + (index * vec->item_size);
  return p;
}

void main() {
  vector vec = v_new(sizeof(int));
  for (int i = 1; i < 21; i++) {
    v_push(&vec, &i);
  }
  int insert_val = 21;
  v_insert(&vec, 10, &insert_val);
  v_delete(&vec, 15);
  for (int i = 0; i < 20; i++) {
    int *val = (int *)v_get(&vec, i);
    printf("%d\n", *val);
  }
}
