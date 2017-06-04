#include<stdio.h>
#include<unistd.h>
#include<sys/types.h>
#include<errno.h>
#include<stdlib.h>

int main(){
    pid_t pid;
    pid = fork();
    if(!pid){
        while(1){
            printf("Daemon running\n");
            sleep(30);
        }
    }
    if(pid > 0)
        exit(0);
    if(pid == -1)
        perror("error in fork\n");
}
