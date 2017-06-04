/* This is singly linked list implementation program.
Only head and right(next) pointer is used*/

#include<stdio.h>
#include<stdlib.h>
#include "list.h"

int main(){
  char ch;
  int num,key;
  lnodeptr ptr;
  head = NULL;/*initializing head ptr*/
  printf("Insert(i)/Search(s)/Delete(d)/Exit(e) : ");
  while((ch=getchar()) != 'e'){
    switch(ch){
    case 'i':
      printf("Enter no. of nodes:");
      scanf("%d",&num);
      for(;num>0;num--){
        printf("Enter key:");
        scanf("%d",&key);
        if(head != NULL)
          insertNode(createNode(key));
        else
          head = createNode(key);
      }
      printf("Head : %d,\tList : ",head->key);
      printList(head);
      printf("\n");
      break;
    case 's':
      printf("Key value to be searched:");
      scanf("%d",&key);
      if((ptr=searchNode(key)) != NULL)
        printf("Found : %d\t Stored at : %p\n",ptr->key,ptr);
        else
          printf("Oopss !! No such key found\n");
      break;
    case 'd':
      printf("Key value to be deleted:");
      scanf("%d",&key);
      if((ptr=searchNode(key)) != NULL){
        key=deleteNode(ptr);
        printf("Node deleted\n");
        printf("Head : %d\nList : ",head->key);
        printList(head);
        //printf("\n");
      }
      else
        printf("Oopss !! No such key exists\n");
      break;
    default:
      printf("Wrong choice, try again\n");
      break;
    }
    while((ch=getchar()) != '\n' && ch != EOF);
    printf("Insert(i)/Search(s)/Delete(d)/Exit(e) : ");
  }
  return 0;
}
