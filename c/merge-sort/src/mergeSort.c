#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define bool	_Bool

typedef int (* Comparer)(void*, void*);

int str_comparer(void* a, void* b) {
  return strcmp((char*)a, (char*)b);
}

void merge_sort(void** array, int start, int end, Comparer comparer) {
  int size = end - start + 1;
  if (size == 1) {
    return;
  }
  int middle = (start + end) / 2;
  int a = start;
  int b = middle + 1;
  merge_sort(array, a, middle, comparer);
  merge_sort(array, b, end, comparer);
  void* work[size];
  for(int i = 0; i < size; i++) {
    if (b > end || (a <= middle && comparer(array[a], array[b]) < 1)) {
      work[i] = array[a];
      a++;
    } else {
      work[i] = array[b];
      b++;
    }
  }
  for (int i = 0; i < size; i++) {
    array[start + i] = work[i];
  }
  return;
}

void main() {
  char* fruits[] = {
    "mango", "banana", "pineapple", "coconut",
    "melon", "peach", "grape", "tomato",
    "lime", "plum", "cherry", "apple",
    "kiwifruit", "grapefruit", "raspberry", "strawberry",
    "guava", "lemon", "mangosteen", "papaya",
    "pear", "blackberry", "soursop", "orange",
    "watermelon", "olive", "lychee", "dragon fruit",
    "passion fruit", "persimmon", "avocado"
  };
  int size = sizeof(fruits) / sizeof(char*);
  merge_sort((void**)fruits, 0, size - 1, str_comparer);
  for (int i = 0; i < size; i++) {
    printf("%d: %s\n", i, (char*)fruits[i]);
  }
}
