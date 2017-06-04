/*
Write a C program orphan.c to create an orphan process and verify using ps(1) command

You can search it by 'ps -ef | grep -i orphan' => orphan process' parent PID should be 1 i.e. init process
*/

#include<stdio.h>
#include<unistd.h>
#include<sys/wait.h>
#include<stdlib.h>

int main(){
    pid_t pid;
    printf("== Parent PID %d ==\n",getpid());
    pid = fork();
    if(pid > 0){/*parent*/
        printf("Exiting from parent\n");
        exit(EXIT_SUCCESS);
    }
    if(pid == -1){
        perror("fork");
        exit(EXIT_FAILURE);
    }
    if(!pid){
        printf("Child PID %d\n",getpid());
        pause();
    }
}
