#include<stdio.h>
#include<unistd.h>
#include<sys/types.h>
#include<errno.h>
#include<stdlib.h>

int main(){
    int i,stat;
    pid_t pid;
    for(i=1;i<17;i++){
        pid = fork();
        if(!pid){ /*child process*/
            printf("Child %d pid : %d\n",i,getpid());
            exit(0);
        }
        if(pid > 0){/*parent process*/
            wait(&stat);
            printf("My child pid : %d, and exit status : %d\n",pid,stat);
        }
        if(pid == -1)
            perror("error in fork\n");
    }
    return 0;
}
