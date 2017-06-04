/* this program reverse a given linked.
linked list is created randomly*/

#include<stdio.h>
#include<stdlib.h>
#include "list.h"

lnodeptr createRandomList(int nodes){
  for(;nodes>0;nodes--){
    if(head != NULL)
      insertNode(createNode(rand()));
    else
      head = createNode(rand());
  }
  return head;
}

lnodeptr reverseList(lnodeptr *hd){/* head should be passed*/
  lnodeptr prev = NULL;
  lnodeptr nxt = (*hd)->right;
  while((*hd) != NULL){
    (*hd)->right = prev;
    prev = *hd;
    *hd = nxt;
    if(nxt != NULL)
      nxt = nxt->right;  
  }
  /* at this point head points to NULL so re-assignment below*/
  *hd = prev;
  return *hd;
}

int main(){
  int nodes;
  head = NULL;
  printf("Enter number of nodes:");
  scanf("%d",&nodes);
  printf("List : ");
  //createRandomList(nodes);
  printList(createRandomList(nodes));
  printf("Head : %d\n",head->key);
  printf("After reversing : ");
  //reverseList();
  //printf("headptr : %p\n",head);
  printList(reverseList(&head));
  printf("Head : %d\n",head->key);
  return 0;
}
