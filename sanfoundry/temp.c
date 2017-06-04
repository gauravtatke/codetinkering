#include<stdio.h>
#include<unistd.h>
#include<string.h>

int main(int argc, char *argv[]){
    int i=0,j;
    int pipex[2];
    char *arg1[][4] = {{"cat","copy.c",NULL,NULL},{"grep","if",NULL,NULL}};
    char *arg2[] = {"ls","-l",NULL};
    //char arg3[5];
    //printf("args::%s\n",argv[0]);
    int outfd = dup(1);
    pipe(pipex);
    for(i=0;i<2;i++)
        for(j=0;j<4;j++)
            printf("args::%s\n",arg1[i][j]);
    //dup2(pipex[1],1);
    pid_t cpid = fork();
    if(!cpid){
        dup2(pipex[1],1);
        close(pipex[0]);
        execvp(arg1[0][0],arg1[0]);
    }
    if(cpid>0){
        wait(0);
        printf("child exec\n");
        dup2(pipex[0],0);
        close(pipex[1]);
        printf("dup exec\n");
        //dup2(outfd,1);
        execvp(arg1[1][0],arg1[1]);
    }
    return 0;
}
