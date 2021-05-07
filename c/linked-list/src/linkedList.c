#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define bool	_Bool

typedef struct item {
  void* value;
  struct item* next;
  struct item* prev;
} Item;

Item* item_new(void* val, size_t size) {
  Item* p = (Item*)malloc(sizeof(Item));
  void* vp = malloc(size);
  memcpy(vp, val, size);
  p->value = vp;
  p->next = NULL;
  p->prev = NULL;
  return p;
}

Item* item_new_with_next(void* val, size_t size, Item* next) {
  Item* p = item_new(val, size);
  if (next != NULL) {
    Item* prev = next->prev;
    if (prev != NULL) {
      prev->next = p;
    }
    p->next = next;
    p->prev = prev;
    next->prev = p;
  }
  return p;
}

Item* item_new_with_prev(void* val, size_t size, Item* prev) {
  Item* p = item_new(val, size);
  if (prev != NULL) {
    Item* next = prev->next;
    if (next != NULL) {
      next->prev = p;
    }
    p->next = next;
    p->prev = prev;
    prev->next = p;
  }
  return p;
}

void item_free(Item* i) {
  free(i->value);
}


typedef struct {
  Item* head;
  Item* tail;
  int size;
  size_t item_size;
} LinkedList;

LinkedList ll_new(size_t size) {
  LinkedList list;
  list.size = 0;
  list.head = NULL;
  list.tail = NULL;
  list.item_size = size;
  return list;
}

bool is_empty(const LinkedList* list) {
  return list->size == 0;
}

int ll_size(const LinkedList* list) {
  return list->size;
}

Item* ll_get_item(const LinkedList* list, int index) {
  Item* p = list->head;
  while(index) {
    if (p->next == NULL) {
      return NULL;
    }
    p = p->next;
    index--;
  }
  return p;
}

void* ll_get(const LinkedList* list, int index) {
  Item* p = ll_get_item(list, index);
  return p->value;
}

void ll_insert_front(LinkedList* list, Item* next, void* val) {
  Item* p = item_new_with_next(val, list->item_size, next);
  if (p->prev == NULL) {
    list->head = p;
  }
  if (p->next == NULL) {
    list->tail = p;
  }
  list->size++;
}

void ll_insert_back(LinkedList* list, Item* prev, void* val) {
  Item* p = item_new_with_prev(val, list->item_size, prev);
  if (p->prev == NULL) {
    list->head = p;
  }
  if (p->next == NULL) {
    list->tail = p;
  }
  list->size++;
}

void ll_push_front(LinkedList* list, void* val) {
  ll_insert_front(list, list->head, val);
}

void ll_push_back(LinkedList* list, void* val) {
  ll_insert_back(list, list->tail, val);
}

void ll_insert(LinkedList* list, int index, void* val) {
  Item* next = ll_get_item(list, index);
  int* next_val = (int*)next->value;
  ll_insert_front(list, next, val);
}

void ll_delete(LinkedList* list, int index) {
  Item* p = ll_get_item(list, index);
  Item* prev = p->prev;
  Item* next = p->next;
  if (prev == NULL) {
    list->head = next;
  } else {
    prev->next = next;
  }
  if (next == NULL) {
    list->tail = prev;
  } else {
    next->prev = prev;
  }
  item_free(p);
  list->size--;
}

void* ll_pop_front(LinkedList* list) {
  Item* p = list->head;
  list->head = p->next;
  list->size--;
  return p->value;
}

void* ll_pop_back(LinkedList* list) {
  Item* p = list->tail;
  list->tail = p->prev;
  list->size--;
  return p->value;
}

void ll_free(LinkedList* list) {
  Item* p = list->head;
  while(p != NULL) {
    item_free(p);
    p = p->next;
  }
}

void main() {
  LinkedList list = ll_new(sizeof(int));
  for (int i = 1; i < 11; i++) {
    ll_push_front(&list, &i);
  }
  for (int i = 11; i < 21; i++) {
    ll_push_back(&list, &i);
  }
  int insert_val = 21;
  ll_insert(&list, 10, &insert_val);
  ll_delete(&list, 8);

  int* pop_fv = (int*)ll_pop_front(&list);
  int* pop_fb = (int*)ll_pop_back(&list);
  printf("pop front: %d back: %d \n", *pop_fv, *pop_fb);

  for (int i = 0; i < ll_size(&list); i++) {
    printf("get %d: ", i);
    int *val = (int *)ll_get(&list, i);
    printf("%d\n", *val);
  }
  ll_free(&list);
}
