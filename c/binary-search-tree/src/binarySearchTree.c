#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#define bool	_Bool

typedef struct node {
  void* value;
  struct node* parent;
  struct node* left;
  struct node* right;
} Node;

Node* node_new(void* value) {
  Node* n = (Node*)malloc(sizeof(Node));
  n->value = value;
  n->parent = NULL;
  n->left = NULL;
  n->right = NULL;
  return n;
}

Node* node_add_left(Node* parent, Node* node) {
  parent->left = node;
  node->parent = parent;
}

Node* node_add_right(Node* parent, Node* node) {
  parent->right = node;
  node->parent = parent;
}

Node* node_get_min(Node* p) {
  while(p->left != NULL) {
    p = p-> left;
  }
  return p;
}

Node* node_get_max(Node* p) {
  while(p->right != NULL) {
    p = p-> right;
  }
  return p;
}

typedef int (* Comparer)(Node*, Node*);

int str_comparer(Node* a, Node* b) {
  char* a_val = (char*)a->value;
  char* b_val = (char*)b->value;
  return strcmp(a_val, b_val);
}

typedef struct {
  Node* root;
  Comparer comparer;
  int size;
} Tree;

Tree bst_new(Comparer comparer) {
  Tree t;
  t.root = NULL;
  t.comparer = comparer;
  t.size = 0;
  return t;
}

void _bst_insert_node(Tree* t, Node* current, Node* n) {
  if (current == NULL) {
    t->root = n;
    return;
  }
  int result = t->comparer(current, n);
  if (result > 0) {
    if (current->left == NULL) {
      node_add_left(current, n);
      return;
    }
    return _bst_insert_node(t, current->left, n);
  } else {
    if (current->right == NULL) {
      node_add_right(current, n);
      return;
    }
    return _bst_insert_node(t, current->right, n);
  }
}

void bst_insert_node(Tree* t, Node* n) {
  n->right = NULL;
  n->left = NULL;
  n->parent = NULL;
  _bst_insert_node(t, t->root, n);
  t->size++;
}

Node* bst_get_min(Tree* t) {
  return node_get_min(t->root);
}

Node* bst_get_max(Tree* t) {
  return node_get_max(t->root);
}

Node* _bst_get(Tree* t, Node* target, Node* value) {
  int result = t->comparer(target, value);
  if (result > 0) {
    return _bst_get(t, target->left, value);
  } else if (result < 0) {
    return _bst_get(t, target->right, value);
  } else {
    return target;
  }
}

Node* bst_get(Tree* t, void* value) {
  return _bst_get(t, t->root, node_new(value));
}

void _bst_to_array(Node* target, Node** array, int* len) {
  if (target->left != NULL) {
    _bst_to_array(target->left, array, len);
  }
  array[(*len)++] = target;
  if (target->right != NULL) {
    _bst_to_array(target->right, array, len);
  }
}

void bst_delete(Tree* t, void* value) {
  Node* target = bst_get(t, value);
  if (target == NULL) {
    return;
  }
  bool isRoot = target->parent == NULL;
  bool isLeft = !isRoot && target->parent->left == target;
  if (target->left == NULL && target->right == NULL) {
    if (isRoot) {
      t->root = NULL;
    } else if (isLeft) {
      target->parent->left = NULL;
    } else {
      target->parent->right = NULL;
    }
  } else if (target->left == NULL) {
    if (isLeft) {
      node_add_left(target->parent, target->right);
    } else {
      node_add_right(target->parent, target->right);
    }
  } else if (target->right == NULL) {
    if (isLeft) {
      node_add_left(target->parent, target->left);
    } else {
      node_add_right(target->parent, target->left);
    }
  } else {
    Node* max = node_get_max(target->left);
    max->parent->right = NULL;
    if (max->left != NULL) {
      node_add_right(max->parent, max->left);
    }
    node_add_left(max, target->left);
    node_add_right(max, target->right);
    if (isLeft) {
      node_add_left(target->parent, max);
    } else {
      node_add_right(target->parent, max);
    }
  }
  free(target);
  t->size--;
}

Node** bst_to_array(Tree* t) {
  Node** array = malloc(t->size * sizeof(Node*));
  int len = 0;
  _bst_to_array(t->root, array, &len);
  return array;
}

void _bst_rebalance(Tree* t, Node** array, int start, int end) {
  int binary = (start + end + 1) / 2;
  if (start > end) {
    return;
  }
  Node* binary_node = array[binary];
  bst_insert_node(t, binary_node);
  _bst_rebalance(t, array, start,  binary - 1);
  _bst_rebalance(t, array, binary + 1, end);
}

void bst_rebalance(Tree* t) {
  Node** array = bst_to_array(t);
  int end = t->size - 1;
  t->root = NULL;
  t->size = 0;
  _bst_rebalance(t, array, 0, end);
  free(array);
}

void bst_insert(Tree* t, void* value) {
  Node* n = node_new(value);
  bst_insert_node(t, n);
}

void bst_print_node(Node* target) {
  if (target->left != NULL) {
    bst_print_node(target->left);
  }

  Node* prev = target;
  Node* p = target->parent;
  char tmp[1024] = "\0";
  while (p != NULL) {
    int len = strlen((char*)p->value);
    memset(tmp + strlen(tmp), ' ', len);
    if (p->parent != NULL) {
      if ((p->parent->left == p && p->left != prev) ||
          (p->parent->right == p && p->right != prev)) {
        strcat(tmp, " | ");
      } else {
        strcat(tmp, "   ");
      }
    }
    tmp[strlen(tmp)] = 0;
    prev = p;
    p = p->parent;
  }

  char prefix[1024] = "";
  for (int i = 0; i < strlen(tmp); i++) {
    prefix[i] = tmp[strlen(tmp) - i - 1];
  }
  if (target->parent != NULL) {
    if (target->parent->left == target) {
      strcat(prefix, "  _");
    } else {
      strcat(prefix, " |_");
    }
  }
  char* suffix = target->left == NULL && target->right == NULL
    ? ""
    : target->left != NULL ? "_|" : "_";
  printf("%s%s%s\n", prefix, (char*)target->value, suffix);

  if (target->right != NULL) {
    bst_print_node(target->right);
  }
}

void bst_print_tree(Tree* t) {
  printf("\n");
  bst_print_node(t->root);
  printf("\n");
}

void main() {
  Tree t = bst_new(str_comparer);
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
  for (int i = 0; i < sizeof(fruits) / sizeof(char*); i++) {
    bst_insert(&t, fruits[i]);
  }

  printf("size: %d\n", t.size);
  printf("min: %s\n", (char*)bst_get_min(&t)->value);
  printf("max: %s\n", (char*)bst_get_max(&t)->value);

  Node** array = bst_to_array(&t);
  for (int i = 0; i < t.size; i++) {
    printf("%s\n", (char*)array[i]->value);
  }

  bst_print_tree(&t);

  bst_rebalance(&t);

  bst_print_tree(&t);

  bst_insert(&t, "jackfruit");
  bst_print_tree(&t);

  bst_delete(&t, "lemon");
  bst_print_tree(&t);
}
