#include<stdio.h>
#include<sys/types.h>
#include<sys/msg.h>
#include<sys/ipc.h>
#include<string.h>
#include<stdlib.h>

#define READ 0
#define WRITE 1

int rdwr = READ;

int main(){
    int key = ftok("key",5);
    char data[] = "loop";
    struct my_msgq{
        long mtype;
        char mtext[10];
    };
    struct my_msgq msg;//send message struct
    struct my_msgq rcv;//receive message struct
    int ret;
    msg.mtype = 1;
    strcpy(msg.mtext,data);
    int msgq_id = msgget(key,IPC_CREAT|0666);
    pid_t cpid;
    size_t msgsize = sizeof(strlen(data)); 
    size_t bufsize = sizeof(strlen(msg.mtext));
    cpid = fork();
    //printf("msg size %ld\n",sizeof(strlen(data)));
    if(!cpid){/*child code*/
        printf("Child PID :: %d\n",getpid());
        printf("rdwr :: %d\n",rdwr);
        while(1){
            if((ret = msgrcv(msgq_id,&rcv,bufsize,0,IPC_NOWAIT)) > 0){
                printf("return val: %d messg receive :: %s\n",ret, rcv.mtext);
                sleep(1);
            }
            //if(rdwr == READ)
            //  continue;
            else{
                perror("No read\n");
                exit(EXIT_SUCCESS);
            }
     }
    }
    if(cpid > 0){/*parent code*/
        //char ch[] = "ch";
        int i=0;
        rdwr = WRITE;
        printf("Parent PID :: %d\n",getpid());
        for(i;i<5;i++){
            if(msgsnd(msgq_id,&msg,msgsize,IPC_NOWAIT) == -1){
              perror("msgsnd\n");
              wait(0);
              exit(EXIT_FAILURE);
            }
            else
                printf("msg send\n");
      }
        wait(0);
    }
    if(cpid == -1){
        perror("Fork\n");
        exit(EXIT_FAILURE);
      }
    return 0;
}
