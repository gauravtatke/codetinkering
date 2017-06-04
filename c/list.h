#include "node.h"

typedef struct node lnode;
typedef struct node *lnodeptr;
typedef struct sp_node spnode;

#define nodesize sizeof(lnode)
#define isListEmpty(ptr) ((ptr == NULL)?1:0)


lnodeptr searchPrevNode(lnodeptr);
lnodeptr createNode(int);
void insertBeforeNode(lnodeptr,lnodeptr);
void printList(lnodeptr);
lnodeptr searchNode(int);
void insertNode(lnodeptr);
void insertAfterNode(lnodeptr,lnodeptr);
int deleteNode(lnodeptr);
lnodeptr searchPrevNode(lnodeptr);
int sp_print(spnode *);
int sp_insert(int);
spnode *sp_create(int);
int sp_delete(spnode *);
spnode *sp_search(int);
int sp_reverse();
