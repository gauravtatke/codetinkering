/* a template of node to be used in linked list, tree etc. */

struct node{ /* A node */
  int key; /* key value */
  struct node *right; /* pointer to right(next) node in list */
  struct node *left; /* ponter to left(previous) node. will be used only if
                   it is doubly linked list */
} *head,*tail; /* pointer to first and last node */

struct sp_node{/*node to be used in single pointer linked list */
    int key;
    struct sp_node *np;
} *sp_head, *sp_tail;
