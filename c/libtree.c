#include<stdio.h>
#include<stdlib.h>

typedef struct tnode *treeptr
typedef struct node{
  int key;
  treeptr prnt; /*parent pointer*/
  treeptr left; /*left child pointer*/
  treeptr right;/*right child pointer*/
}tnode;

treeptr root = NULL;
treeptr loc;/*location to add new node,always points to a leaf node*/

treeptr createTNode(int keyval){
  /*creates node, sets key, return pointer to node*/
  treeptr tmp;
  if((tmp = (treeptr *)malloc(sizeof(tnode))) == NULL)
    printf("Cannot create node: Insufficient memory\n");
  else{
    tmp->key = keyval;
    return tmp;
  }
}

treeptr insertNode(treeptr *lf,treeptr nd){
  if((*lf->prnt)->right == NULL){/*add as right sibling*/
    (*lf->prnt)->right = nd;
    nd->prnt = *lf;
    nd->left = nd->right = NULL;
  }
}
treeptr createTree(int numnode){
  /*creates tree with given # nodes*/
  for(;numnode>0;numnode--){
    if(root != NULL){/*non-empty tree*/

    }
  }
}
