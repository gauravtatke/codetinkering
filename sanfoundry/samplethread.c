#include<pthread.h>
#include<stdio.h>
#include<stdlib.h>

void *thread_fn(void *arg){
	printf("in the thread %u\n",(unsigned int)pthread_self());
	//exit(0);
	//return (void *)0;
	pthread_exit(0);
}

/*int main(){
	pthread_t tid;
	int i,j;
	pthread_create(&tid,NULL,thread_fn,NULL);
	pthread_join(tid,NULL);
	return 0;
}*/

int main(){
	pthread_t tid[25];
	int k, j;
	for(k=0;k<25;k++){
		pthread_create(&tid[k],NULL,thread_fn,NULL);
	}

	for(j=0;j<25;j++){
		pthread_join(tid[j],NULL);
	}
return 0;
}
