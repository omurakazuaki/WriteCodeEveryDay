#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

int index_of(int *array, int start, int end, int val) {
  int binary = (end + start + 1) / 2;
  int binary_val = array[binary];
  if (binary_val == val) {
    return binary;
  }
  if (start > end) {
    return -1;
  }
  int new_start = binary_val > val ? start : binary + 1;
  int new_end = binary_val > val ? binary - 1 : end;
  return index_of(array, new_start, new_end, val);
}

int _fib(int* array, int n) {
  if (n == 0 || n == 1) {
    array[n] = 1;
  }
  if (array[n] != 0) {
    return array[n];
  }
  return array[n] = _fib(array, n - 1) + _fib(array, n - 2);

}

int* fib(int n) {
  int* array = calloc(n + 1, sizeof(int));
  _fib(array, n);
  return array;
}

void main() {
  const int N = 40;
  int* array = fib(N);
  for (int i = 1; i < N + 1; i++) {
    printf("fib %d: %d\n", i, array[i]);
    printf("indexOf: %d\n", index_of(array, 1, N + 1, array[i]));
    assert(i == index_of(array, 0, N + 1, array[i]));
  }
  assert(-1 == index_of(array, 1, N + 1, 56));
}
