/*
Program to reverse the contents of a file
 */

#include<stdio.h>
#include<unistd.h>
#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>

int main(int argc, char *argv[]){
    int fdrd,fdwr,curloc;
    char ch,eol;
    if (argc != 3){
        fprintf(stderr,"Usage: rvsl <sourcefile> <destination file>\n");
        return 0;
    }
    else{
        if ((fdrd = open(argv[1],O_RDONLY)) >= 0){
            fdwr = open(argv[2],O_WRONLY|O_CREAT,0755);
            curloc = lseek(fdrd,-1,SEEK_END);
            read(fdrd,&eol,1); /*reads the end of line character*/
            lseek(fdrd,-1,SEEK_CUR);
            for(curloc;curloc>0;curloc--){
                lseek(fdrd,-1,SEEK_CUR);
                read(fdrd,&ch,1);
                write(fdwr,&ch,1);
                lseek(fdrd,-1,SEEK_CUR);
            }
            write(fdwr,&eol,1);
            close(fdrd);
            close(fdwr);
        }
    }
    return 0;
}
