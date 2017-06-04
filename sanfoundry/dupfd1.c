/*
Write a C program dupfd1.c which performs the following:
-  opens a file for writing (say fd is fdnew)
-  duplicates the file descriptor 1 to fdnew descriptor.
-  Subsequently the program reads from the standard input and writes to the standard output.
 */

#include<stdio.h>
#include<unistd.h>
#include<stdlib.h>
#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>

int main(){
    int fdnew;
    char *ch;
    if((fdnew = open("temp",O_WRONLY|O_CREAT|O_TRUNC)) != -1){//open successful
        if(dup2(fdnew,1) == -1){/*duplicating fdnew to stdout, so anything written to stdout will go to temp file*/
            perror("dup2");
            exit(EXIT_FAILURE);
        }
    }
    else{
        perror("open");
        exit(EXIT_FAILURE);
    }
/*
  Now we can take input from stdin and write to stdout.
  Anything goes to stdout actually goes to temp file
  as stdout is now duplicate of fdnew.
*/
    scanf("%s",ch);
    printf("Input from STDIN: %s\n",ch);
    close(fdnew);
    return 0;
}
