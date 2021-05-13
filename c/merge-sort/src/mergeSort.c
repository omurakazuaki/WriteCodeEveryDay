#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define bool	_Bool

typedef int (* Comparer)(void*, void*);

int str_comparer(void* a, void* b) {
  return strcmp((char*)a, (char*)b);
}

void merge(void** array, int left, int right, int end, Comparer comparer) {
  int size = end - left + 1;
  void* work[size];
  int start = left;
  for(int i = 0; i < size; i++) {
    if (right > end || (left <= right - 1 && comparer(array[left], array[right]) < 1)) {
      work[i] = array[left];
      left++;
    } else {
      work[i] = array[right];
      right++;
    }
  }
  for (int i = 0; i < size; i++) {
    array[start + i] = work[i];
  }
}

void merge_sort(void** array, int start, int end, Comparer comparer) {
  if (start == end) {
    return;
  }
  int middle = (start + end) / 2;
  int left = start;
  int right = middle + 1;
  merge_sort(array, left, middle, comparer);
  merge_sort(array, right, end, comparer);
  merge(array, left, right, end, comparer);
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
