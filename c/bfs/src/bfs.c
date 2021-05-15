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

void* v_get(Vector* v, size_t index) {
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

typedef struct graph_t {
  Vector* vertices;
  Vector* edges;
} Graph;

Graph g_new() {
  Graph g;
  g.vertices = v_new();
  g.edges = v_new();
  return g;
}

void g_push(Graph* g, void* vertice) {
  v_push(g->vertices, vertice);
  Vector* edges = v_new();
  v_push(g->edges, edges);
}

void g_add_directed_edge(Graph* g, size_t* from, size_t* to) {
  Vector* edges = v_get(g->edges, *from);
  v_push(edges, to);
}

void g_add_edge(Graph* g, size_t* a, size_t* b) {
  g_add_directed_edge(g, a, b);
  g_add_directed_edge(g, b, a);
}


int bfs(Graph* g, size_t from, size_t to) {
  Vector* queue = v_new();
  int dist[g->vertices->size];
  for(size_t i = 0; i < g->vertices->size; i++) {
    dist[i] = -1;
  }
  dist[from] = 0;
  v_push(queue, &from);
  while (!v_is_empty(queue)) {
    size_t* v = v_shift(queue);
    Vector* edges = (Vector*)v_get(g->edges, *v);
    for (size_t i = 0; i < edges->size; i++) {
      size_t* next = v_get(edges, i);
      if (dist[*next] == -1) {
        v_push(queue, next);
        dist[*next] = dist[*v] + 1;
        printf("%ld: dist=%d\n", *next, dist[*next]);
        if (*next == to) {
          return dist[*next];
        }
      }
    }
  }
  return -1;
}

void main() {
  int vals[] = {0, 1, 4, 2, 3, 8, 5, 7, 6};
  size_t edges[][2] = {
    {0, 1}, {0, 2}, {0, 3},
    {1, 2}, {1, 4}, {1, 5},
    {2, 5},
    {3, 6},
    {4, 5}, {4, 7},
    {5, 6},
    {6, 8},
    {7, 8}
  };
  /*
    0----1---3---7
    | \ / \ /    |
    |  4---8     |
    |      |     |
    2------5 ----6
  */
  int vals_size = sizeof(vals) / sizeof(int);
  int edges_size = sizeof(edges) / sizeof(size_t) / 2;
  Graph g = g_new();
  for (int i = 0; i < vals_size; i++) {
    g_push(&g, &vals[i]);
  }
  for (int i = 0; i < edges_size; i++) {
    g_add_edge(&g, &edges[i][0], &edges[i][1]);
  }
  int dist = bfs(&g, 0, 8);
  printf("0 to 8 dist=%d\n", dist);

  dist = bfs(&g, 5, 0);
  printf("5 to 0 dist=%d\n", dist);
}
