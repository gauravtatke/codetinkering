/*
Write a C program intio.c that reads an integer string from STDIN, increments it and then displays the new value to STDOUT
Use only read()/write() system calls.
Sample Inputs:100, 25700, 65536
Expected Outputs: 101, 25701, 65537
*/

#include<stdio.h>
#include<unistd.h>
#include<stdlib.h>
#include<string.h>
#include<limits.h>
#include<errno.h>

#define MAXLEN 20 //max length of long int

void reverse(char source[]);
void itoa(long int value, char dest[]);

int main(){
	int rd,wr,base = 10;
    long int val;
	char buff[MAXLEN];
    char **endptr;

    errno = 0;

	rd = read(0,&buff,MAXLEN);
	//printf("long max : %ld\n",LONG_MAX);
    val = strtol(buff,endptr,base);

    //(**endptr != '\0' && **endptr != '\n') ? printf("End Ptr:%s\n",**endptr):printf("NULL\n");

    if(errno == ERANGE || (errno != 0 && val == 0)){/*error occured*/
        perror("strtol");
        exit(EXIT_FAILURE);
    }

    if(*endptr == buff){/*no digits found*/
        fprintf(stderr,"Either no digits were found OR input contained chars\n");
        exit(EXIT_FAILURE);
    }

    if(val == LONG_MAX){
        fprintf(stderr,"Cannot add 1: result will overflow\n");
        exit(EXIT_FAILURE);
    }

    val += 1;
    itoa(val,buff);
	wr = write(1,&buff,strlen(buff));
    return 0;
}

void reverse(char s[]){
    int c, i, j;
    for (i = 0,j = strlen(s)-1; i< j; i++, j--){
        c = s[i];
        s[i] = s[j];
        s[j] = c;
    }
}

void itoa(long int n, char *ptr){
    int i;
    long int sign;
    if((sign = n) < 0)
        n = -n;
    i = 0;
    do{
        ptr[i++] = n%10 + '0';
    }while ((n /= 10) > 0);
    if(sign < 0)
        ptr[i++] = '-';
    ptr[i] = '\0';
    reverse(ptr);
}
