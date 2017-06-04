/*
Write a C program copyf.c that reads input from the keyboard and displays the content on the monitor using copy function.
function: int copy(int fd_src, int fd_dst);
- The function copies the contents of the file represented by the descriptor fd_src, to the file represented by the descriptor 
fd_dst.
- The function should use only system calls.
- The function returns 0 if the operation is successful, else returns -1.
 */

#include<unistd.h>
#include<stdio.h>
#include<stdlib.h>

#define MAXLEN 64 /*max input length*/

int copy(int src_fd,int dest_fd);

int main(){
    if(copy(0,1) < 0){
        perror("copy");
        exit(EXIT_FAILURE);
    }
    return 0;
}

int copy(int src,int dest){
    int i;
    char buff[MAXLEN];
    while((i=read(src,&buff,MAXLEN)) > 0){
        i = write(dest,&buff,i);
        if(i == -1)/*Error*/
            return -1;
    }
    if(i == -1)/*Error*/
        return -1;
    return 0;
}
