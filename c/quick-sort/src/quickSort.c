#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#define bool	_Bool

typedef int (* Comparer)(void*, void*);

int str_comparer(void* a, void* b) {
  return strcmp((char*)a, (char*)b);
}

void* med3(void** array, int start, int end, Comparer comparer) {
  int size = end - start + 1;
  void* v1 = array[rand() % size + start];
  void* v2 = array[rand() % size + start];
  void* v3 = array[rand() % size + start];
  if (comparer(v1, v2) > 1) {
    return comparer(v2, v3) > 1 ? v2: v3;
  }  else {
    return comparer(v1, v3) > 1 ? v1: v3;
  }
}

void quick_sort(void** array, int start, int end, Comparer comparer) {
  if (start >= end) {
    return;
  }
  int middle = (start + end) / 2;
  void* p = med3(array, start, end, comparer);
  int left = start;
  int right = end;
  void* left_val = NULL;
  void* right_val = NULL;
  while(left <= right) {
    if (comparer(p, array[left]) < 1) {
      left_val = array[left];
    } else {
      left++;
    }
    if (comparer(p, array[right]) >= 1) {
      right_val = array[right];
    } else {
      right--;
    }
    if (right_val != NULL && left_val != NULL) {
      array[left++] = right_val;
      array[right--] = left_val;
      left_val = NULL;
      right_val = NULL;
    }
  }
  quick_sort(array, start, left - 1, comparer);
  quick_sort(array, left, end, comparer);
  return;
}

void main() {
  srand((unsigned int)time(NULL));
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
  quick_sort((void**)fruits, 0, size - 1, str_comparer);
  for (int i = 0; i < size; i++) {
    printf("%d: %s\n", i, (char*)fruits[i]);
  }
}
