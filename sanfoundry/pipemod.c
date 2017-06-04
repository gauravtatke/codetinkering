/*
 *Program to simulate pipe '|'. It can take max upto 10 args and each arg can have
 *upto 10 options such as 
 *./pipe "ls -l -r -t ..<upto 10>" "grep -i -v .. <upto 10>" ... <upto 10>
 */

#include<string.h>
#include<stdio.h>
#include<stdlib.h>
#include<unistd.h>

#define MAXARG 10
#define MAXOPT 10

#define READ 0
#define WRITE 1

//char *argstore[MAXARG][MAXOPT+1];

int splitarg(int count,char *argarr[],char *matrix[][MAXOPT+1]){
    int i,j;
    char *str1;
    if(count < 3 || count > MAXARG+1){
        fprintf(stderr,"Usage: pipe <cmd1 in double quote> <cmd2 in double quotes>...upto %d args\n",MAXARG);
        exit(EXIT_FAILURE);
    }
    for(i=1;i<count;i++){
        for(str1 = argarr[i],j=0;j<MAXOPT+1;str1 = NULL,j++){
            if((matrix[i-1][j] = strtok(str1," ")) == NULL)
                break;
        }
        if(j == MAXOPT+1){
            fprintf(stderr,"Arg %s has more than %d options, reduce and try again..\n",argarr[i],MAXOPT);
            exit(EXIT_FAILURE);
        }
    }
    return 0;
}

int setpipe(int arnum, int arcnt, int (*parr)[2]){
    if(arnum == 0){//first arg reads from STDIN writes to pipe
        if(dup2(parr[arnum][WRITE],1) == -1)
            return -1;
        close(parr[arnum][READ]);
    }
    else if(arnum+2 == arcnt){//last arg reads from previous pipe and outputs to STDOUT
        if(dup2(parr[arnum-1][READ],0) == -1)
            return -1;
        close(parr[arnum-1][WRITE]);
    }
    else{//reads from previous pipe and outputs to the next pipe
        if(dup2(parr[arnum-1][READ],0) == -1)
            return -1;
        if(dup2(parr[arnum][WRITE],1) == -1)
            return -1;
        close(parr[arnum-1][WRITE]);
        close(parr[arnum][READ]);
    }
    return 1;
}

int main(int argc, char *argv[]){
    int i,j,stat;
    char *str;
    int pipex[MAXARG-1][2];
    pid_t cpid;
    char *argstore[MAXARG][MAXOPT+1];
    splitarg(argc,argv,argstore);
    for(i=0;i<MAXARG-1;i++){//create pipex[0-8]
        if(pipe(pipex[i]) == -1){
            perror("pipe::");
            exit(EXIT_FAILURE);
        }
    }
    for(i=0;i<argc-1;i++){
        cpid = fork();
        if(!cpid){//Child process executes the arguements
            if(setpipe(i,argc,pipex) == -1){
                perror("setpipe():");
                exit(EXIT_FAILURE);
                }
            else
                execvp(argstore[i][0],argstore[i]);
            }
        if(cpid == -1){
            perror("fork\n");
            exit(EXIT_FAILURE);
        }
        if(cpid>0){
            //parent closes the write end of the pipe in which
            //child is writing so that next arg reading it
            //gets EOF
            close(pipex[i][WRITE]);
            wait(&stat);
        }
    }
    /*Parent closes all ends of all pipes as it doesn't need
and waits for all the processes it created*/
    //    for(j=0;j<MAXARG-1;j++){
    //  close(pipex[j][READ]);
    //  close(pipex[j][WRITE]);
    // }
    //for(j=0;j<argc-1;j++)
    //  wait(&stat);
    return 0;
}

