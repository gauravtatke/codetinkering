#include<unistd.h>
#include<signal.h>
#include<sys/time.h>
#include<stdio.h>

void f1(){
    printf("\nThis is a segmentation fault handler\n");
}

int main(){
    int num, ret;
    struct sigaction a;
    a.sa_handler = f1;
    a.sa_flags = SA_RESETHAND;
    ret = sigaction(SIGSEGV, &a, NULL);
    printf("\nEnter a number  ");
    scanf("%d",&num);
    printf("\n%d\n",ret);
    return 0;
}
