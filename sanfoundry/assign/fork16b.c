/*
3 programs to achieve same things. Parent creates 16 processes
and wait for each of them to terminate.Please un-comment the program 
you want to run and comment out others
*/

#include<stdio.h>
#include<unistd.h>
#include<stdlib.h>
#include<sys/wait.h>
#include<sys/types.h>

/*
PROGRAM - 1
One parent process creates 16 child processes.
Each child prints its PID and exits.
Parent waits for each of the child process to exit and 
print its PID and exit status. Parent exits itself once there's
no child in waitable state.
 */


int main(){
    pid_t pid,retpid;
    int i,stats;
    printf("== Parent PID :: %d ==\n\n",getpid());
    for(i=0;i<16;i++){
        pid = fork();
        if(!pid){
            printf("Child %d PID::%d\n",i+1,getpid());
            exit(EXIT_SUCCESS);
        }
        if(pid == -1){
            perror("Fork\n");
            exit(EXIT_FAILURE);
        }
    }
    sleep(10);
    while(1){
        if((retpid = wait(&stats)) > 0)
            printf("Child exited :: %d with exit status :: %d\n",retpid,stats);
        else{
            printf("No child to exit\n");
            exit(EXIT_SUCCESS);
        }
    }
    return 0;
}


/*
PROGRAM - 2
One parent creates 16 child processes.Each child print its PID and waits for a signal.
Parent waits for each of the child to change the state (TERMINATED, STOP, RESUME).
Parent print the child PID and the change in the state. Parent exits itself
once there's no child in waitable state, i.e. all child exited. 
*/
/*
int main(){
    pid_t pid,retpid;
    int i,stats;
    printf("== Parent PID :: %d ==\n\n",getpid());
    for(i=0;i<16;i++){
        pid = fork();
        if(!pid){
            printf("Child %d PID::%d\n",i+1,getpid());
            if(getpid()%5 == 0)
                exit(EXIT_SUCCESS);
            pause(); //Use SIGSTOP and SIGCONT signals to stop and resume the process
        }
        if(pid == -1){
            perror("Fork\n");
            exit(EXIT_FAILURE);
        }
        sleep(1);
    }
    while(1){
        if((retpid = waitpid(-1,&stats,WUNTRACED|WCONTINUED)) > 0){
            if(WIFEXITED(stats))
                printf("Child exited :: %d normally with exit status :: %d\n",retpid,WEXITSTATUS(stats));
            if(WIFSIGNALED(stats))
                printf("Child :: %d terminated by signal :: %d\n",retpid,WTERMSIG(stats));
            if(WIFSTOPPED(stats))
                printf("Child :: %d stopped by signal :: %d\n",retpid,WSTOPSIG(stats));
            if(WIFCONTINUED(stats))
                printf("Child :: %d resumed by signal :: 18\n",retpid);
        }
        else{
            printf("No child remains, exiting\n");
            exit(EXIT_SUCCESS);
        }
    }
}
*/

/*
PROGRAM - 3
Same as above but using waitid() function
*/
/*
int main(){
    pid_t pid;
    int i;
    struct siginfo sginf;
    printf("== Parent PID :: %d ==\n\n",getpid());
    for(i=0;i<3;i++){
        pid = fork();
        if(!pid){
            printf("Child %d PID::%d\n",i+1,getpid());
            if(getpid()%5 == 0)
                exit(EXIT_SUCCESS);
            pause();
        }
        if(pid == -1){
            perror("Fork\n");
            exit(EXIT_FAILURE);
        }
        sleep(1);
    }
    while(1){
        if(!waitid(P_ALL,0,&sginf,WEXITED|WSTOPPED|WCONTINUED)){
            if(sginf.si_code == CLD_EXITED)
                printf("Child exited :: %d normally with exit status :: %d\n",sginf.si_pid,sginf.si_status);
            if(sginf.si_code == CLD_KILLED)
                printf("Child :: %d terminated by signal :: %d\n",sginf.si_pid,sginf.si_status);
            if(sginf.si_code == CLD_STOPPED)
                printf("Child :: %d stopped by signal :: %d\n",sginf.si_pid,sginf.si_status);
            if(sginf.si_code == CLD_CONTINUED)
                printf("Child :: %d resumed by signal :: %d\n",sginf.si_pid,sginf.si_status);
        }
        else{
            printf("No child remains, exiting\n");
            exit(EXIT_SUCCESS);
        }
    }
}
*/
