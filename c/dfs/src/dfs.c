#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define bool	_Bool

typedef struct vector_t {
  void** raw;
  int size;
  int capacity;
} Vector;

Vector* v_new() {
  Vector* v = malloc(sizeof(Vector));
  v->capacity = 8;
  v->size = 0;
  v->raw = malloc(v->capacity * sizeof(void*));
  return v;
}

bool v_is_empty(Vector* v) {
  return v->size == 0;
}

void v_resize(Vector* v) {
  if (v->size == v->capacity) {
    v->capacity = v->capacity << 1;
    v->raw = realloc(v->raw, v->capacity * sizeof(void*));
  }
}

void* v_get(const Vector* v, const size_t index) {
  return v->raw[index];
}

void* v_shift(Vector* v) {
  void* val = v->raw[0];
  v->size--;
  memcpy(v->raw, v->raw + 1, v->size * sizeof(void*));
  return val;
}

void* v_pop(Vector* v) {
  v->size--;
  void* val = v->raw[v->size];
  v->raw[v->size] = NULL;
  return val;
}


void v_push(Vector* v, void* val) {
  v_resize(v);
  v->raw[v->size] = val;
  v->size++;
}

size_t* find_unvisited_node(char map[17][17], size_t* from, bool visited[16][16]) {
  for (size_t i = from[0] - 1; i < from[0] + 2; i++) {
    for (size_t j = from[1] - 1; j < from[1] + 2; j++) {
      if (i != from[0] && j != from[1]) continue;
      if (map[i][j] == '=') continue;
      if (visited[i][j]) continue;
      size_t* pos = malloc(sizeof(size_t) * 2);
      pos[0] = i; pos[1] = j;
      return pos;
    }
  }
  return NULL;
}

Vector* dfs(char map[17][17], size_t* from, size_t* to) {
  Vector* stack = v_new();
  bool visited[16][16];
  for(size_t i = 0; i < 16; i++) {
    for(size_t j = 0; j < 16; j++) {
      visited[i][j] = 0;
    }
  }
  visited[from[0]][from[1]] = 1;
  v_push(stack, from);
  while (!v_is_empty(stack)) {
    size_t* v = v_get(stack, stack->size - 1);
    if (v[0] == to[0] && v[1] == to[1]) {
      return stack;
    }
    size_t* unvisited = find_unvisited_node(map, v, visited);
    if (unvisited != NULL) {
      visited[unvisited[0]][unvisited[1]] = 1;
      v_push(stack, unvisited);
    } else {
      size_t* v = v_pop(stack);
      free(v);
    }
  }
  return stack;
}

Vector* bfs_route(Vector* stack) {
  Vector* result = v_new();
  size_t* v = v_pop(stack);
  v_push(result, v);
  while (!v_is_empty(stack)) {
    size_t* cur = v_get(result, result->size - 1);
    size_t* next = v_pop(stack);
    int x_delta = abs(cur[0] - next[0]);
    int y_delta = abs(cur[1] - next[1]);
    if ((x_delta == 1 && y_delta == 0) ||
        (x_delta == 0 && y_delta == 1)) {
      v_push(result, next);
    }
  }
  return result;
}

Vector* bfs(char map[17][17], size_t* from, size_t* to) {
  Vector* queue = v_new();
  Vector* visitedVec = v_new();
  bool visited[16][16];
  for(size_t i = 0; i < 16; i++) {
    for(size_t j = 0; j < 16; j++) {
      visited[i][j] = 0;
    }
  }
  visited[from[0]][from[1]] = 1;
  v_push(queue, from);
  while (!v_is_empty(queue)) {
    size_t* v = v_shift(queue);
    v_push(visitedVec, v);
    for (size_t i = v[0] - 1; i < v[0] + 2; i++) {
      for (size_t j = v[1] - 1; j < v[1] + 2; j++) {
        if (i != v[0] && j != v[1]) continue;
        if (map[i][j] == '=') continue;
        if (visited[i][j]) continue;
        visited[i][j] = 1;
        size_t* pos = malloc(sizeof(size_t) * 2);
        pos[0] = i; pos[1] = j;
        v_push(queue, pos);
        if (pos[0] == to[0] && pos[1] == to[1]) {
          return bfs_route(visitedVec);
        }
      }
    }
  }
  return bfs_route(visitedVec);
}

void main() {
  char map[17][17] = {
    "================",
    "=s             =",
    "= ==== ======= =",
    "= =    =     = =",
    "= = ======== = =",
    "= =        =   =",
    "= === === ==== =",
    "=       = =  = =",
    "= ======= == = =",
    "= =        = = =",
    "= ========   = =",
    "=        = === =",
    "======== = =   =",
    "=g =   = = =====",
    "==   =   =     =",
    "================"
  };
  size_t* from = malloc(sizeof(size_t) * 2);
  size_t* to = malloc(sizeof(size_t*) * 2);
  for (int i = 1; i < 15; i++) {
    for (int j = 1; j < 15; j++) {
      if (map[i][j] == 's') {
        from[0] = i; from[1] = j;
      } else if (map[i][j] == 'g') {
        to[0] = i; to[1] = j;
      }
    }
  }

  Vector* route = dfs(map, from, to);
  for (int i = 0; i < route->size; i++) {
    size_t* pos = v_get(route, i);
    if ((from[0] == pos[0] && from[1] == pos[1]) ||
        (to[0]   == pos[0] && to[1]   == pos[1])) continue;
    map[pos[0]][pos[1]] = '*';
  }
  for (int i = 0; i < 16; i++) {
    printf("%s\n", map[i]);
  }

  printf("\n");

  char map2[17][17] = {
    "================",
    "=s             =",
    "= ==== ======= =",
    "= =    =     = =",
    "= = ======== = =",
    "= =        =   =",
    "= === === ==== =",
    "=       = =  = =",
    "= ======= == = =",
    "= =        = = =",
    "= ========   = =",
    "=        = === =",
    "======== = =   =",
    "=g =   = = =====",
    "==   =   =     =",
    "================"
  };
  Vector* route2 = bfs(map2, from, to);
  for (int i = 0; i < route2->size; i++) {
    size_t* pos = v_get(route2, i);
    if ((from[0] == pos[0] && from[1] == pos[1]) ||
        (to[0]   == pos[0] && to[1]   == pos[1])) continue;
    map2[pos[0]][pos[1]] = '*';
  }
  for (int i = 0; i < 16; i++) {
    printf("%s\n", map2[i]);
  }

}
