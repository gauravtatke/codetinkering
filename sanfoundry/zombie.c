#include<stdio.h>
#include<unistd.h>
#include<sys/types.h>
#include<errno.h>
#include<stdlib.h>

int main(){
    pid_t pid;
    int status;
    char ch;
    pid = fork();
    if(!pid){
        printf("Zombie process:%d\n",getpid());
        return 1;
    }
    if(pid > 0){
        printf("Enter any char:");
        ch = getchar();
        wait(&status);
    }
    if(pid == -1)
        perror("Error in fork\n");
    return 0;
}
