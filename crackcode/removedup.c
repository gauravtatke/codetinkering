#include <stdio.h>
#include <string.h>

void shiftleft(char loc[], int shiftcount){
    char *prev, *curr;
    while(shiftcount--){
        prev = loc;
        curr = prev+1;
        while((*prev++ = *curr++));
    }
}

char *rem_dup1(char *str){
    char *start = str;
    char *loc;
    while(*start){
        if((loc = strchr(str, *start)) == start){
            //no duplicate
            start++;
        }
        else
            shiftleft(start, 1);
    }
    return str;
}

char *rem_dup2(char str[]){
    /*This is crack code method*/
    int len = strlen(str);
    int tail = 1;
    int i, j;
    if(len == 0)
        return NULL;
    if(len < 2)
        return str;
    for(i = 0; i < len; i++){
        for(j = 0; j < tail; j++){
            if(str[i] == str[j])
                break; //dup found, break out
        }
        if(j == tail)//copy unique char at tail and increment tail
            str[tail++] = str[i];
    }
    str[tail] = '\0';
    return str;
}

int main(int argc, char *argv[]){
    printf("after removing dup = %s\n", rem_dup2(argv[1]));
    return 0;
}
