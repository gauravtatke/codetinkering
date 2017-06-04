/* This is stack implementation using linked list.
Only head pointer is used as top pointer */

#include<stdio.h>
#include<stdlib.h>
#include "list.h"

void push(int key){
/*implementing PUSH using functions from lnklist.h*/
  if(head != NULL)
    insertNode(createNode(key));
  else
    head = createNode(key);
}

int pop(){
  if(isListEmpty(head)){
    printf("Stack Underflow\n");
    exit(1);
  }
  else
    return deleteNode(head);
}

int main(){
  char ch;
  int val;
  head = NULL; /* initialization of head to NULL */
  printf("Enter p(U)sh/p(O)p/(E)xit : ");
  while((ch=getchar()) != 'E' && ch != 'e'){
    switch(ch){
    case 'U':
    case 'u':
      printf("Enter value to push : ");
      scanf("%d",&val);
      push(val);
      printf("Value pushed!!\n");
      printf("Top : %d\tStack : ",head->key);
      printList(head);
      break;
    case 'O':
    case 'o':
      printf("Value popped : %d\n",pop());
      if(!isListEmpty(head))
        printf("Top : %d\tStack : ",head->key);
      printList(head);
      break;
    default:
      printf("Incorrect choice, try again\n");
      break;
    }
    //printf("test\n");
  while((ch=getchar()) != '\n' && ch != EOF);
  printf("Enter p(U)sh/p(O)p/(E)xit : ");
  }
  return 0;
}
