#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define bool	_Bool

const int TABLE_SIZE = 10000;

int str_hash_code(char* str) {
  int sum = 0;
  int i = 0;
  while(str[i] != 0) {
    sum += str[i++];
  }
  //printf("hash %s=%d\n", str, sum);
  return sum % TABLE_SIZE;
}

typedef struct item {
  char* key;
  char* value;
  struct item* next;
} Item;

Item* item_new(char* key, char* val, Item* next) {
  Item* p = (Item*)malloc(sizeof(Item));
  p->key = key;
  p->value = val;
  if (next != NULL) {
    p->next = next;
  } else {
    p->next = NULL;
  }
  return p;
}

void item_free(Item* i) {
  free(i->key);
  free(i->value);
}

Item* item_find(Item* head, char* key) {
  Item* i = head;
  while(i != NULL) {
    if (strcmp(i->key, key) == 0) {
      return i;
    }
    i = i->next;
  }
  return NULL;
}

typedef struct {
  Item** bucket;
  int size;
} HashTable;

HashTable ht_new() {
  HashTable ht;
  ht.bucket = calloc(TABLE_SIZE, sizeof(Item*));
  ht.size = 0;
  return ht;
}

bool ht_exists(HashTable* ht, char* key) {
  int hash = str_hash_code(key);
  Item* head = ht->bucket[hash];
  Item* find = item_find(head, key);
  return find != NULL;
}

void ht_put(HashTable* ht, char* key, char* val) {
  int hash = str_hash_code(key);
  Item* head = ht->bucket[hash];
  Item* find = item_find(head, key);
  if (find == NULL) {
    Item* next = head;
    Item* new_item = item_new(key, val, next);
    ht->bucket[hash] = new_item;
    ht->size++;
  } else {
    find->value = val;
  }
}

char* ht_get(HashTable* ht, char* key) {
  int hash = str_hash_code(key);
  Item* head = ht->bucket[hash];
  Item* find = item_find(head, key);
  if (find == NULL) {
    return NULL;
  }
  return find->value;
}

void ht_remove(HashTable* ht, char* key) {
  int hash = str_hash_code(key);
  Item* head = ht->bucket[hash];

  Item* i = head;
  Item* prev = NULL;
  while(i != NULL) {
    if (strcmp(i->key, key) == 0) {
      ht->size--;
      if (prev == NULL) {
        ht->bucket[hash] = i->next;
      } else {
        prev->next = i->next;
        ht->bucket[hash] = prev;
      }
      return;
    }
    prev = i;
    i = i->next;
  }
}

void main() {
  HashTable ht = ht_new();
  ht_put(&ht, "emerald", "Kiwifruit");
  ht_put(&ht, "red", "apple");
  ht_put(&ht, "yellow", "banana");
  ht_put(&ht, "green", "melon");
  ht_put(&ht, "Gzz", "test");
  printf("size: %d\n", ht.size);

  char* apple = ht_get(&ht, "red");
  printf("red: %s\n", apple);
  char* test = ht_get(&ht, "Gzz");
  printf("Gzz: %s\n", test);

  char* banana = ht_get(&ht, "yellow");
  printf("yellow: %s\n", banana);

  ht_put(&ht, "yellow", "orange");
  char* orange = ht_get(&ht, "yellow");
  printf("yellow: %s\n", orange);
  printf("size: %d\n", ht.size);

  ht_remove(&ht, "red");
  printf("exists red?: %d\n", ht_exists(&ht, "red"));
  printf("exists Gzz?: %d\n", ht_exists(&ht, "Gzz"));
  printf("size: %d\n", ht.size);

  ht_remove(&ht, "Gzz");
  printf("exists Gzz?: %d\n", ht_exists(&ht, "Gzz"));
  printf("size: %d\n", ht.size);
}
