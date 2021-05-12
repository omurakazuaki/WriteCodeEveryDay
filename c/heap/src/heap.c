#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define bool	_Bool

typedef int (* Comparer)(void*, void*);

int str_comparer(void* a, void* b) {
  char* a_val = (char*)a;
  char* b_val = (char*)b;
  return strcmp(a_val, b_val);
}

typedef struct {
  void** raw;
  Comparer comparer;
  size_t capacity;
  size_t size;
} Heap;

Heap h_new(Comparer comparer) {
  Heap heap;
  heap.capacity = 8;
  heap.comparer = comparer;
  heap.raw = (void**)calloc(heap.capacity, sizeof(void*));
  heap.size = 0;
  return heap;
}

bool is_empty(const Heap* h) {
  return h->size == 0;
}

int h_size(const Heap* h) {
  return h->size;
}

void h_resize(Heap* h) {
  if (h->capacity == h->size) {
    h->capacity = h->capacity << 1;
    h->raw = realloc(h->raw, h->capacity * sizeof(char*));
    memset(h->raw[h->size], 0, h->capacity >> 1 * sizeof(char*));
  }
}

void* h_get(const Heap* h, const int index) {
  if (index < 0 || index >= h->size) {
    return NULL;
  }
  return h->raw[index];
}

bool h_is_left(const int index) {
  return index % 2 == 1;
}

bool h_is_right(const int index) {
  return index != 0 && index % 2 == 0;
}

int h_left_index(const int index) {
  return index * 2 + 1;
}

int h_right_index(const int index) {
  return index * 2 + 2;
}

bool h_is_node(const Heap* h, const int index) {
  return h_left_index(index) >= h->size;
}

int h_parent_index(const int index) {
  if (h_is_left(index)) {
    return (index - 1) / 2;
  } else {
    return (index - 2) / 2;
  }
}

void h_sift_up(Heap* h, int index) {
  if (index == 0) {
    return;
  }
  int parent_index = h_parent_index(index);
  void* val = h_get(h, index);
  void* p_val = h_get(h, parent_index);
  int result = h->comparer(p_val, val);
  if (result >= 0) {
    return;
  }
  h->raw[parent_index] = val;
  h->raw[index] = p_val;
  h_sift_up(h, parent_index);
}

void h_sift_down(Heap* h, int index) {
  void* val = h_get(h, index);
  int left_index = h_left_index(index);
  void* left = h_get(h, left_index);
  if (left == NULL) {
    return;
  }
  int right_index = h_right_index(index);
  void* right = h_get(h, right_index);
  if (right == NULL || h->comparer(left, right) > 0) {
    int resultL = h->comparer(left, val);
    if (resultL > 0) {
      h->raw[left_index] = val;
      h->raw[index] = left;
      h_sift_down(h, left_index);
      return;
    }
  } else {
    if (right == NULL) {
      return;
    }
    int resultR = h->comparer(right, val);
    if (resultR > 0) {
      h->raw[right_index] = val;
      h->raw[index] = right;
      h_sift_down(h, right_index);
      return;
    }
  }
}

void h_insert(Heap* h, void *val) {
  h_resize(h);
  int index = h->size;
  h->raw[index] = val;
  h->size++;
  h_sift_up(h, index);
}

void h_remove(Heap* h, int index) {
  void* last = h_get(h, h->size - 1);
  h->raw[h->size] = NULL;
  h->size--;
  if (h->size == index) {
    return;
  }
  h->raw[index] = last;
  h_sift_down(h, index);
  if (h_get(h, index) == last) {
    h_sift_up(h, index);
  }
}

void* v_extract_max(Heap* h) {
  if (h->size == 0) {
    return NULL;
  }
  void* max = h_get(h, 0);
  h_remove(h, 0);
  return max;
}

Heap heapify(void** array, int size, Comparer comparer) {
  Heap h = h_new(comparer);
  for(int i = 0; i < size; i++) {
    h_insert(&h, array[i]);
  }
  return h;
}

void h_print_node(Heap* h, int index) {
  int left_index = h_left_index(index);
  int right_index = h_right_index(index);
  void* left = h_get(h, left_index);
  void* right = h_get(h, right_index);

  if (left != NULL) {
    h_print_node(h, left_index);
  }

  int prev_index = index;
  int parent_index = h_parent_index(index);
  char tmp[1024] = "\0";
  while (parent_index > -1) {
    int len = strlen((char*)h_get(h, parent_index));
    memset(tmp + strlen(tmp), ' ', len);
    if (h_parent_index(parent_index) > -1) {
      if ((h_is_left(parent_index) && h_left_index(parent_index) != prev_index) ||
          (h_is_right(parent_index) && h_right_index(parent_index) != prev_index)) {
        strcat(tmp, " | ");
      } else {
        strcat(tmp, "   ");
      }
    }
    tmp[strlen(tmp)] = 0;
    prev_index = parent_index;
    parent_index = h_parent_index(parent_index);
  }

  char prefix[1024] = "";
  for (int i = 0; i < strlen(tmp); i++) {
    prefix[i] = tmp[strlen(tmp) - i - 1];
  }

  if (h_is_left(index)) {
    strcat(prefix, "  _");
  } else if (h_is_right(index)) {
    strcat(prefix, " |_");
  }

  char* suffix = left == NULL && right == NULL
    ? ""
    : left != NULL ? "_|" : "_";
  char* value = (char*)h_get(h, index);
  printf("%s%s%s\n", prefix, (char*)value, suffix);

  if (right != NULL) {
    h_print_node(h, right_index);
  }
}

void h_print_tree(Heap* h) {
  printf("\n");
  h_print_node(h, 0);
  printf("\n");
}

void** heap_sort(void** array, int size, Comparer comparer) {
  Heap h = heapify(array, size, comparer);
  void** sorted = malloc(h.size * sizeof(void*));
  void* item = v_extract_max(&h);
  for(int i = size - 1; i > -1; i--) {
    sorted[i] = item;
    item = v_extract_max(&h);
  }
  return sorted;
}

void main() {
  char* fruits[] = {
    "apple", "banana", "blackberry", "coconut",
    "melon", "peach", "grape", "tomato",
    "mango", "lime", "plum", "cherry",
    "kiwifruit", "grapefruit", "raspberry", "strawberry",
    "guava", "lemon", "mangosteen", "papaya",
    "pear", "pineapple", "soursop", "orange",
    "watermelon", "olive", "lychee", "dragon fruit",
    "passion fruit", "persimmon", "avocado"
  };
  int size = sizeof(fruits) / sizeof(char*);
  void** sorted = heap_sort((void**)fruits, size, str_comparer);
  for (int i = 0; i < size; i++) {
    printf("%d: %s\n", i, (char*)sorted[i]);
  }

  Heap h = heapify((void**)fruits, size, str_comparer);
  h_print_tree(&h);

  h_remove(&h, 0);
  h_print_tree(&h);

  h_remove(&h, 5);
  h_print_tree(&h);

  h_remove(&h, 28);
  h_remove(&h, 27);
  h_remove(&h, 16);
  h_print_tree(&h);
}
