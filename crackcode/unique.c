#include <stdio.h>

#define MAXLEN 255

int findunique(char *str){
    char strarr[MAXLEN];
    int i;
    for(i = 0; i <= 255; i++){
        strarr[i] = '\0';
    }

    while(*str){
        if(strarr[(int)(*str)] == '\0'){
            strarr[(int)(*str)] = *str;
            str++;
        }
        else{
            //printf("inside else\n char = %c", *str);
            return 0;
        }
    }
    if(*str == '\0'){
        //printf("inside if\n");
        return 1;
    }
    return 0;
}

int main(int argc, char *argv[]){
    printf("result is %d\n", findunique(argv[1]));
    return 0;
}


