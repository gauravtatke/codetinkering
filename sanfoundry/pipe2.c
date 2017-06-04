#include<stdio.h>
#include<unistd.h>
#include<sys/types.h>
#include<sys/stat.h>
#include<fcntl.h>
#include<stdlib.h>

int send_srcfile_pipe(char *srcfile, int wrend){/*reads from source file, sends to pipe, return num of bytes written to pipe*/
    char buff[64];
    int rd,src;
    if((src = open(srcfile,O_RDONLY)) == -1){
        perror("Cannot open source file :file may not exist\n");
        _exit(EXIT_FAILURE);
    }
    else{
        while((rd = read(src,&buff,64)) > 0)
            write(wrend,&buff,rd);
        //printf("Read source complete\n");
    }
    close(src);
    return 0;
}

int read_pipe_dstfile(int rdend, char *dstfile){/*reads from pipe,writes in destination file, return num of bytes read from pipe*/
    char buff[64];
    int rd,dst;
    if((dst = open(dstfile,O_WRONLY|O_CREAT|O_APPEND,0755)) == -1){
        perror("Cannot open destination file\n");
        _exit(EXIT_FAILURE);
    }
    else{
        while((rd = read(rdend,&buff,64)) > 0)
            write(dst,&buff,rd);
    }
    close(dst);
    return 0;
}

int main(int argc, char *argv[]){
    int pipefd[2];
    pid_t pid;
    if(argc != 3){
        fprintf(stderr,"Usage: %s <srcfile> <destfile>\n",argv[0]);
        _exit(EXIT_FAILURE);
    }
    if(pipe(pipefd) == -1){
        perror("Pipe\n");
        _exit(EXIT_FAILURE);
    }
    
    pid = fork();

    if(!pid){/*Child code*/
        close(pipefd[1]);//child closes write end of pipe
        read_pipe_dstfile(pipefd[0],argv[2]);
        //printf("Nothing more to read from pipe\n");
        _exit(EXIT_SUCCESS);
    }
    if(pid > 0){/*Parent code */
        close(pipefd[0]);//parent closes read end of pipe
        send_srcfile_pipe(argv[1],pipefd[1]);
        //printf("Source file read complete..waiting for child to exit\n");
        close(pipefd[1]);//Child will read EOF because of this
        wait(0);
    }
    if(pid == -1){
        perror("Fork\n");
        _exit(EXIT_FAILURE);
    }
    return 0;    
}
