#include <stdio.h>
#include <string.h>

char *reverse(char *str){
    int i, j, temp;
    for(i = 0, j = strlen(str)-1; i < j; i++, j--){
        temp = str[i];
        str[i] = str[j];
        str[j] = temp;
    }
    return str;
}

int main(int argc, char *argv[]){
    char *str = reverse(argv[1]);
    //char st[] = "gaurav";
    printf("original str = %s\n", argv[1]);
    printf("reverse str = %s\n", reverse(str));
    printf("original str = %s\n", argv[1]);
    return 1;
}
