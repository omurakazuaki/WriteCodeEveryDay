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

vector v_new(const size_t size) {
  vector vec;
  vec.capacity = 8;
  vec.item_size = size;
  vec.raw = (void*)calloc(vec.capacity, vec.item_size);
  vec.size = 0;
  return vec;
}

bool is_empty(const vector* vec) {
  return vec->size == 0;
}

int v_size(const vector* vec) {
  return vec->size;
}

void v_resize(vector* vec) {
  if (vec->capacity == vec->size) {
    vec->capacity = vec->capacity << 1;
    vec->raw = realloc(vec->raw, vec->capacity * vec->item_size);
    memset(vec->raw + (vec->size * vec->item_size), 0, vec->capacity >> 1 * vec->item_size);
  }
}

void v_push(vector* vec, const void *val) {
  v_resize(vec);
  void *p = vec->raw + (vec->size * vec->item_size);
  memcpy(p, val, vec->item_size);
  vec->size += 1;
}

void v_insert(vector* vec, const int index, const void *val) {
  v_resize(vec);
  void *p = vec->raw + (index * vec->item_size);
  memcpy(p + vec->item_size, p, (vec->size - index) * vec->item_size);
  memcpy(p, val, vec->item_size);
  vec->size += 1;
}

void v_delete(vector* vec, const int index) {
  void *p = vec->raw + (index * vec->item_size);
  memcpy(p, p + vec->item_size, (vec->size - index) * vec->item_size);
  vec->size -= 1;
}

void* v_get(const vector* vec, const int index) {
  void *p = vec->raw + (index * vec->item_size);
  return p;
}

void* v_pop(vector* vec) {
  void *p = (void*)calloc(1, vec->item_size);
  memcpy(p, v_get(vec, vec->size - 1), vec->item_size);
  v_delete(vec, vec->size - 1);
  return p;
}

void v_free(vector* vec) {
  free(vec->raw);
}

void hex_dmp(const void *buf, const int size) {
  int i, j;
  unsigned char *p = (unsigned char *)buf, tmp[20];
  printf("+0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +A +B +C +D +E +F|  -- ASCII --\n");
  printf("--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+----------------\n");
  for (i = 0; p - (unsigned char *)buf < size; i++) {
    for (j = 0; j < 16; j++) {
      tmp[j] = (unsigned char)((*p < 0x20 || *p >= 0x7f) ? '.' : *p);
      printf("%02X ", (int)*p);
      if (++p - (unsigned char *)buf >= size) {
        tmp[++j] = '\0';
        for (;j < 16; j++) {
          printf("   ");
        }
        break;
      }
    }
    tmp[16] = '\0';
    printf("%s\n", tmp);
    if (p - (unsigned char *)buf >= size) {
      break;
    }
  }
}

void v_hex_dmp(const vector* vec) {
  hex_dmp(vec->raw, vec->size * vec->item_size);
}

void main() {
  vector vec = v_new(sizeof(int*));
  for (int i = 1; i < 21; i++) {
    v_push(&vec, &i);
  }
  int insert_val = 21;
  v_insert(&vec, 10, &insert_val);
  v_delete(&vec, 15);
  int *pop_val = (int*)v_pop(&vec);
  printf("pop: %d\n", *pop_val);
  for (int i = 0; i < v_size(&vec); i++) {
    int *val = (int *)v_get(&vec, i);
    printf("get %d: %d\n", i, *val);
  }
  v_hex_dmp(&vec);
  v_free(&vec);
  free(pop_val);

  vector str_vec = v_new(sizeof(char**));
  char *str1 = "Kiwifruit";
  v_push(&str_vec, &str1);
  char *str2 = "banana";
  v_push(&str_vec, &str2);
  char *str3 = "orange";
  v_push(&str_vec, &str3);
  char *str4 = "apple";
  v_push(&str_vec, &str4);
  for (int i = 0; i < v_size(&str_vec); i++) {
    char **val = (char **)v_get(&str_vec, i);
    printf("get %d: %s\n", i, *val);
  }
  char **pop_str_val = (char **)v_pop(&str_vec);
  printf("pop: %s\n", *pop_str_val);
  v_hex_dmp(&str_vec);
  v_free(&str_vec);
  free(pop_str_val);
}
