/* This is queue implementation using singly linked list.
Both head and tail pointers are used to keep delete
operation under O(1) time constraint */

#include<stdio.h>
#include<stdlib.h>
#include "list.h"

void enqueue(int val){
  if(!isListEmpty(head))
    insertAfterNode(tail,createNode(val));
  else
    tail = head = createNode(val);
}

int dequeue(){
  if(isListEmpty(head)){
    printf("Queue Empty\n");
      exit(1);
  }
  else
    return deleteNode(head);
}

int main(){
  char ch;
  int val;
  printf("(e)NQUEUE / (d)EUEUE / (q)UIT : ");
  while((ch=getchar()) != 'q'){
    switch(ch){
    case 'e':
      printf("Enter Value : ");
      scanf("%d",&val);
      enqueue(val);
      printf("Value Enqueued !!\n");
      printf("Head : %d,\tTail : %d\nQueue : ",head->key,tail->key);
      printList(head);
      break;
    case 'd':
      printf("Value removed : %d\n",dequeue());
      if(!isListEmpty(head))
        printf("Head : %d,\tTail : %d\nQueue : ",head->key,tail->key);
      printList(head);
      break;
    default:
      printf("Incorrect choice, try again\n");
    }
    while((ch=getchar()) != '\n' && ch != EOF);
    printf("(e)NQUEUE / (d)EUEUE / (q)UIT : ");
  }
  return 0;
}
