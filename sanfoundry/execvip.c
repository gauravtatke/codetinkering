#include<stdio.h>
#include<unistd.h>

int main(){
    int ret = 0;
	char *data[] = {"cat","copy.c",NULL,"gaurav","tatke"};
	//char *vim = {"vi"}; 
    printf("data :: %s",*data);
    ret = execvp("cat",data);
    	if(ret == -1)
        	perror("Error in exec\n");
    	else
        	printf("Successful\n");
}
