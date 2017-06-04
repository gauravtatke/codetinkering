#include<stdio.h>
#include<unistd.h>
#include<sys/types.h>

int main(){
	int i=0;
	for(i=1;i<5;i++){
		if(!fork()){
			printf("PID of child %d: %d, parent: %d\n",i,getpid(),getppid());
			//printf("PID of parent process: %d\n",getppid());
		}
		else{
			//printf("PID of parent process: %d\n",getppid());
			wait(0);
		}
	}
}
