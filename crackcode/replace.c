#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXLEN 255

char *replace(char o_str[], char repch, char *repstr){
    //char result[MAXLEN];
    int count = 0;
    //char *s = o_str;
    //char *r = repstr;
    int i = 0, j = 0, k = 0;
    int olen = strlen(o_str);
    int replen = strlen(repstr);

    if(olen > MAXLEN){
        perror("original string too large\n");
        return NULL;
    }

    while(i < olen){
        if(o_str[i] == repch)
            count++;
            i++;
    }

    if(olen - count + replen > MAXLEN){
        perror("orig + replacement str are too large\n");
        return NULL;
    }
    char *result = (char *)malloc(olen+replen-count+1);

    for(i = 0, j = 0; i < olen; i++){
        if(o_str[i] == repch){
             for(k = 0; k < replen; k++)
                 result[j++] = repstr[k];
        }
        else
            result[j++] = o_str[i];
    }
    result[j] = '\0';
    return result;
}

int main(){
    char st[] = "gaurav is sexy";
    char *rep = replace(st, ' ', "%20");
    printf("replace = %s\n", rep);
    free(rep);
    return 1;
}
