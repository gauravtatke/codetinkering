#include<stdio.h>
#include<stdlib.h>
//#include<math.h>

#define MAX_VAL 65535
int a[MAX_VAL];

int Memoized_maxDollar(int val){
  int i = 1;
    for (i=1;i<=val;i++){
      a[i] = MAX_VAL;
      }
    return max_Dollar(val);
}

int max_Dollar(int val){
  if (a[val] != MAX_VAL){
    return a[val];
      }
  else if (val > (val/2 + val/3 + val/4)){
    a[val] = val;
    return a[val];
  }
  else{
    a[val] = max_Dollar(val/2)+max_Dollar(val/3)+max_Dollar(val/4);
    return a[val];
  }
}

int main(int argc,char *argv[]){
  /*unsigned int sz = sizeof(int);
    printf("Size:%u, max value:%e\n",sz,pow(2,31));*/
  unsigned int num;
  while(--argc > 0){
    // *++argv;
    if((num=atoi(*++argv)) > MAX_VAL)
      printf("Input too long : %d, Should be less than %d\n",num,MAX_VAL);
    else
      printf("Maximum Dollars exchanged for %d = %d\n",num,Memoized_maxDollar(num));
 }
}
