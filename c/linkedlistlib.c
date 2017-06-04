#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>
//#include "node.h"
#include "list.h"

/*
typedef struct node lnode;
typedef struct node *lnodeptr;

#define nodesize sizeof(lnode)
#define isListEmpty(ptr) ((ptr == NULL)?1:0)

lnodeptr searchPrevNode(lnodeptr);
*/

lnodeptr createNode(int val){
  /* create a node, assing val to key, return its pointer */
  lnodeptr ptr = (lnodeptr)malloc(nodesize);
  if(ptr == NULL){
    printf("Cannot create node : Not enough memory\n");
    return NULL;
      }
  ptr->key = val;
  // ptr->left = NULL;
  return ptr;
}

void insertBeforeNode(lnodeptr loc,lnodeptr ptr){
  /* inserts ptr after node pointed to by loc */
  ptr->right = loc;
  if(loc == head){/* if loc is head */
    head = ptr; /* update head to point to new node */
  }
  else{
    lnodeptr prev = searchPrevNode(loc);/*search previous node to loc*/
    prev->right = ptr;
  }
 }

void printList(lnodeptr hd){
  if(!isListEmpty(hd)){
    lnodeptr ptr;
  for(ptr = hd;ptr != NULL;ptr = ptr->right)
      printf("%d\t",ptr->key);
  }
  else
    printf("---Empty list---");
  printf("\n");
}

lnodeptr searchNode(int val){
  /* searches node based on key value passed and returns pointer to it */
  lnodeptr ptr;
  for(ptr = head;ptr != NULL;ptr = ptr->right){
    if(ptr->key == val)
      return ptr;
  }
  return NULL;
}

void insertNode(lnodeptr ptr){
  /* insert before head by default */
  insertBeforeNode(head,ptr);
}

void insertAfterNode(lnodeptr loc,lnodeptr ptr){
  /* inserts node after node pointed by (*loc) */
  //printf("tail:%p,loc:%p,ptr:%p\n",tail,loc,ptr);
  ptr->right = loc->right;  
  loc->right = ptr;
  // ptr->left = *loc;
  if(ptr->right == NULL)
    tail = ptr;/* tail is updated to point to last node */
  //printf("tail:%p,loc:%p,ptr:%p\n",tail,loc,ptr);
}


int deleteNode(lnodeptr ptr){
  int val=ptr->key;
  if(ptr == head){
    if(ptr->right == NULL){/*if this is the only node*/
      //free(ptr);
      tail = head = NULL;
    }
    else
      head = ptr->right;
  }
  else{
    lnodeptr left = searchPrevNode(ptr);
    left->right = ptr->right;
    if(ptr == tail)
      tail = left;
  }
  free(ptr);
  return val;
}

lnodeptr searchPrevNode(lnodeptr ptr){
  /* searches the previous node of node pointed by ptr */
  if(ptr == head)/*if ptr is head return NULL*/
    return NULL;
  else{
    lnodeptr prev;
    for(prev = head; (prev->right) != ptr; prev=prev->right);
    return prev;
  }
}

/*Below are the functions for single pointer linked list
* only  sp_node is used
*/

spnode *sp_create(int key){
    spnode *ptr = (spnode *)malloc(sizeof(spnode));
    if (ptr == NULL){
        printf("Not enough memory\n");
        return NULL;
    }
    ptr->key = key;
    return ptr;
}

spnode *XOR(spnode *ptr1, spnode *ptr2){
    return (spnode *)((uintptr_t)ptr1 ^ (uintptr_t)ptr2);
}

int sp_insert(int key){
    spnode *new = sp_create(key);
    spnode *next;
    new->np = (spnode *)((uintptr_t)sp_head ^ (uintptr_t)NULL);
    if(sp_tail == NULL){
        sp_tail = new;
    }
    if(sp_head != NULL){
        next = (spnode *)((uintptr_t)(sp_head->np) ^ (uintptr_t)NULL);
        sp_head->np = (spnode *)((uintptr_t)next ^ (uintptr_t)new);
    }
    sp_head = new;
    return 0;
}

int sp_print(spnode *head){
    spnode *prev = NULL;
    spnode *curr = head;
    spnode *next;
    while(curr != NULL){
        printf("%d\t",curr->key);
        next = (spnode *)((uintptr_t)prev ^ (uintptr_t)(curr->np));
        prev = curr;
        curr = next;
    }
    printf("\n");
    return 0;
}

int sp_delete(spnode *nd){
    spnode *curr = sp_head;
    spnode *prev = NULL;
    spnode *prev_prev = NULL;
    spnode *next = NULL;
    spnode *next_next = NULL;
    while(curr != nd){
        next = XOR(curr->np, prev);
        prev = curr;
        curr = next;
    }
    next = XOR(curr->np, prev);
    if(next != NULL){
        next_next = XOR(curr, next->np);
    }
    if(prev != NULL){
        prev_prev = XOR(curr, prev->np);
    }
    if(nd == sp_head){
        /*head to be removed*/
        if(next != NULL){
            /*update only next
            *head is not the only element
            */
            next->np = XOR(next_next, prev);
            sp_head = next;
            return 0;
        }
        /*head is the only element*/
        sp_head = NULL;
        return 0;
    }
    if(next == NULL){/*nd is last node, as ext is null*/
        prev->np = XOR(next, prev_prev);
        sp_tail = prev;
        return 0;
    }
    next->np = XOR(next_next, prev);
    prev->np = XOR(prev_prev, next);
    return 0;
}

spnode *sp_search(int key){
    spnode *curr = sp_head;
    spnode *prev = NULL;
    spnode *next;
    while(curr != NULL){
        if(curr->key == key){
            return curr;
        }
        next = XOR(curr->np, prev);
        prev = curr;
        curr = next;
    }
    return NULL;
}

int sp_reverse(){
    spnode *tmp = sp_head;
    sp_head = sp_tail;
    sp_tail = tmp;
    return 0;
}
