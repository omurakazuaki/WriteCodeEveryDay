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
  return sum % TABLE_SIZE;
}

typedef struct {
  void *raw;
  int size;
} HashTable;

HashTable ht_new() {
  HashTable ht;
  ht.raw = calloc(TABLE_SIZE, sizeof(char*));
  ht.size = 0;
  return ht;
}

bool ht_exists(HashTable* ht, char* key) {
  int hash = str_hash_code(key);
  char* p = (char*)ht->raw + hash * sizeof(char**);
  return p[0] != 0;
}

void ht_put(HashTable* ht, char* key, char* val) {
  int hash = str_hash_code(key);
  if (!ht_exists(ht, key)) {
    ht->size++;
  }
  memcpy(ht->raw + hash * sizeof(char*), val, sizeof(char*));
}

char* ht_get(HashTable* ht, char* key) {
  int hash = str_hash_code(key);
  char* p = (char*)ht->raw + hash * sizeof(char**);
  return p;
}

char* ht_remove(HashTable* ht, char* key) {
  int hash = str_hash_code(key);
  char* p = (char*)ht->raw + hash * sizeof(char**);
  int i = 0;
  while(p[i] != 0) {
    p[i] = 0;
    i++;
  }
}

void main() {
  HashTable ht = ht_new();
  ht_put(&ht, "emerald", "Kiwifruit");
  ht_put(&ht, "red", "apple");
  ht_put(&ht, "yellow", "banana");
  ht_put(&ht, "green", "melon");
  printf("size: %d\n", ht.size);

  char* apple = ht_get(&ht, "red");
  printf("red: %s\n", apple);

  char* banana = ht_get(&ht, "yellow");
  printf("yellow: %s\n", banana);

  ht_put(&ht, "yellow", "orange");
  char* orange = ht_get(&ht, "yellow");
  printf("yellow: %s\n", orange);
  printf("size: %d\n", ht.size);

  printf("exists red?: %d\n", ht_exists(&ht, "red"));
  ht_remove(&ht, "red");
  ht_exists(&ht, "red");
  printf("exists red?: %d\n", ht_exists(&ht, "red"));
}
