#include<stdio.h>
#include<stdlib.h>
#include<error.h>
#include<assert.h>
#include<limits.h>

#define STACKSIZE 5

typedef struct{
    int value;
    int min;
}StackNode;

StackNode *buffer[STACKSIZE];
int top = -1;

int min(){
    if(top < 0){
       return INT_MAX; 
    }
    return buffer[top]->min;
}

void push(int val){
    int localmin;
    if(top+1 >= STACKSIZE){
        perror("Overflow\n");
        exit(EXIT_FAILURE);
    }
    StackNode *node = (StackNode *)malloc(sizeof(StackNode));
    node->value = val;
    localmin = min();
    node->min = (localmin < val ? localmin : val);
    buffer[++top] = node;
}

int pop(){
    if(top < 0){
        perror("Underflow\n");
        exit(EXIT_FAILURE);
    }
    int retval = buffer[top--]->value;
    free(buffer[top+1]);
    return retval;
}

int main(){
    int i, val;
    for(i = 0; i < STACKSIZE; i++){
        val = rand();
        push(val);
        printf("Pushed\t%d\n",val);
    }
    printf("\n########################\n\n");
    for(i = 0; i < STACKSIZE; i++){
        printf("Min before pop\t%d\n", min());
        printf("Pop\t\t\t%d\n", pop());
        printf("Min after pop\t%d\n", min());
    }
    return 0;
}
