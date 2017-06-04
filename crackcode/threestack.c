#include<stdio.h>
#include<error.h>
#include<stdlib.h>
#include<assert.h>

#define STACKSIZE 5
#define NUMSTACKS 3

int buffer[STACKSIZE*3]; //stack to keep elements
int stacktop[] = {-1, -1, -1}; //to keep track of top elements of each stack

struct StackNode{
    int value;
    int lastindex;
};

struct StackNode *buffnode[STACKSIZE*NUMSTACKS];
int indexused = 0;
int stacknodetop[] = {-1, -1, -1};

void pushnode(int stacknum, int value){
    if(stacknum >= NUMSTACKS || stacknum < 0){
        perror("Invalid Stacknum");
        exit(EXIT_FAILURE);
    }
    int index = stacknodetop[stacknum];
    if(indexused >= STACKSIZE*NUMSTACKS){
        perror("Overflow");
        exit(EXIT_FAILURE);
    }
    struct StackNode *node = 
        (struct StackNode *)malloc(sizeof(struct StackNode));
    assert(node != NULL);
    node->value = value;
    node->lastindex = index;
    stacknodetop[stacknum] = indexused;
    buffnode[indexused] = node;
    indexused++;
}


int popnode(int stacknum){
    if(stacknum >= NUMSTACKS || stacknum < 0){
        perror("Invalid Stacknum");
        exit(EXIT_FAILURE);
    }
    if(indexused < 0 || stacknodetop[stacknum] < 0){
        perror("Underflow");
        exit(EXIT_FAILURE);
    }
    int value = buffnode[stacknodetop[stacknum]]->value;
    int index = buffnode[stacknodetop[stacknum]]->lastindex;
    indexused--;
    free(buffnode[stacknodetop[stacknum]]);
    stacknodetop[stacknum] = index;
    return value;
}

void push(int stacknum, int value){
    int index = STACKSIZE*stacknum + stacktop[stacknum] + 1;
    int limit = STACKSIZE*stacknum + STACKSIZE;
    //printf("index = %d, limit = %d\n", index, limit);
    if(index >= limit){
        perror("Overflow for stack");
        exit(EXIT_FAILURE);
    }
    buffer[index] = value;
    stacktop[stacknum]++;
}

int pop(int stacknum){
    int index = STACKSIZE*stacknum + stacktop[stacknum];
    int limit = STACKSIZE*stacknum;
    if(index < limit){
        perror("Underflow");
        exit(EXIT_FAILURE);
    }
    stacktop[stacknum]--;
    return buffer[index];
}

void printstack(){
    int i;
    for(i=0; i<NUMSTACKS*STACKSIZE; i++){
        printf("value %d\n", buffnode[i]->value);
    }
            
}
void testpush(){
    int i;
    int rnum;
    for(i=0; i<NUMSTACKS*STACKSIZE; i++){
        rnum = rand();
        pushnode(rnum%NUMSTACKS, rnum);
    }
}



int main(){
    testpush();
    printstack();
    return 0;
}
