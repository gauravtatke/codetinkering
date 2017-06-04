#include<stdio.h>
#include<signal.h>
#include<unistd.h>

void sighand(){
    printf("Process pid: %d\n",getpid());
}

int main(){
    int ret;
    struct sigaction siga;
    siga.sa_handler = sighand;
    siga.sa_flags = SA_RESETHAND;
    ret = sigaction(SIGINT,&siga,NULL);
    pid_t pid = fork();
    if(!pid){
        //printf("Child process: %d is going sleep\n",getpid());
        sleep(100);
    }
    if(pid > 0){
        printf("Parent PID = %d and Child PID : %d\n",getpid(),pid);
        printf("Sending SIGINT to : %d\n",pid);
        kill(pid,2);
    }
    if(pid == -1)
        perror("Error in fork\n");
    return 0;
}
