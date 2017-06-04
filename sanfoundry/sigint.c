#include<stdio.h>
#include<signal.h>
#include<unistd.h>

int main(){
    printf("Disabling CTRL-c for 10 sec \n");
    signal(SIGINT,SIG_IGN);
    sleep(10);
    printf("Enabling CTRL-c for 10 sec \n");        
    signal(SIGINT,SIG_DFL);
    sleep(10);
    return 0;
}
