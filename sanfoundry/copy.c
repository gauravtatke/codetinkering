/*
Write a program to copy file1 to file2
Input file is vmlinux
 */

#include<stdio.h>
#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>
#include<unistd.h>
#include<stdlib.h>

int main (int argc, char *argv[]){
    int fdrd,fdwr,i,w;
    char ch[64];

    if (argc != 3){
        fprintf(stderr,"Usage:copy <source> <destination>\n");
        exit(EXIT_FAILURE);
    }
    
    if ((fdrd = open(argv[1],O_RDONLY)) >= 0){
        fdwr = open(argv[2],O_WRONLY|O_CREAT,0755);
        while((i = read(fdrd,&ch,64)) > 0){
            w = write(fdwr,&ch,i);
        }
        close(fdrd);
        close(fdwr);
        }
    else{
        fprintf(stderr,"file does not exist\n");
        exit(EXIT_FAILURE);
    }

    return 0;
}
