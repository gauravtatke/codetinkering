/*
Program to simulate pipe '|'. It can take max upto 10 args and each arg can have
upto 10 options such as 
./pipe "ls -l -r -t ..<upto 10>" "grep -i -v .. <upto 10>" ... <upto 10>
 */

#include<string.h>
#include<stdio.h>
#include<stdlib.h>
#include<unistd.h>

#define MAXARG 10
#define MAXOPT 10

#define READ 0
#define WRITE 1

char *argstore[MAXARG][MAXOPT+1];

int splitarg(int count,char *argarr[]){
    int i,j;
    char *str1;
    if(count < 3 || count > MAXARG+1){
        fprintf(stderr,"Usage: pipe <cmd1 in double quote> <cmd2 in double quotes>...upto %d args\n",MAXARG);
        exit(EXIT_FAILURE);
    }
    for(i=1;i<count;i++){
        for(str1 = argarr[i],j=0;j<MAXOPT+1;str1 = NULL,j++){
            if((argstore[i-1][j] = strtok(str1," ")) == NULL)
                break;
        }
        if(j == MAXOPT+1){
            fprintf(stderr,"Arg %s has more than %d options, reduce and try again..\n",argarr[i],MAXOPT);
            exit(EXIT_FAILURE);
        }
    }
    return 0;
}

int main(int argc, char *argv[]){
    int i,j,stat;
    char *str;
    int pipex[2],pipey[2];
    int outfd,infd;
    pid_t cpid;
    splitarg(argc,argv);
    if(pipe(pipex) == -1 || pipe(pipey) == -1){
        perror("pipe\n");
        exit(EXIT_FAILURE);
    }
    for(i=0;i<argc-1;i++){
        cpid = fork();
        if(!cpid){//Child process executes the arguements
            if(i==0){/*first arg,STDOUT is write end of pipe*/
                if(dup2(pipex[WRITE],1) == -1){
                    perror("dup2:");
                    exit(EXIT_FAILURE);
                }
            }
            else if((i%2 == 0) && (i+2 != argc)){//even #no arg, read from pipey and write to pipex 
                if(dup2(pipex[WRITE],1) == -1){
                    perror("dup2:");
                    exit(EXIT_FAILURE);
                }
                if(dup2(pipey[READ],0) == -1){
                    perror("dup2:");
                    exit(EXIT_FAILURE);
                }
            }
            else if((i%2 == 1) && (i+2 != argc)){//odd #no arg, read from pipex and write to pipey
                if(dup2(pipey[WRITE],1) == -1){
                    perror("dup2:");
                    exit(EXIT_FAILURE);
                }
                if(dup2(pipex[READ],0) == -1){
                    perror("dup2:");
                    exit(EXIT_FAILURE);         
                }
            }
            else{/*last arg, STDOUT should be monitor*/
                if(i%2 == 0){
                    if(dup2(pipey[READ],0) == -1){
                        perror("dup2:");
                        exit(EXIT_FAILURE);
                    }
                }
                else{
                    if(dup2(pipex[READ],0) == -1){
                        perror("dup2:");
                        exit(EXIT_FAILURE);
                    }
                }
            }
            close(pipex[WRITE]);
            close(pipex[READ]);
            close(pipey[WRITE]);
            close(pipey[READ]);
            execvp(argstore[i][0],argstore[i]);
        }
        /*if(cpid > 0){
            if(i==0 || i%2 == 0)
                close(pipex[WRITE]);
            else
                close(pipey[WRITE]);
            wait(&stat);
            }*/
    }
    /*Parent closes all ends of all pipes as it doesn't need
and waits for all the processes it created*/
    close(pipex[WRITE]);
    close(pipex[READ]);
    close(pipey[WRITE]);
    close(pipey[READ]);
    for(j=0;j<argc-1;j++)
        wait(&stat);
    return 0;
}
