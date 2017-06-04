#include<stdio.h>

/*Program to count number of occurence of digit in a
  specified range of integers */

int nooccur(int lower,int upper, int find){
  int i,count = 0,x;
  for(lower;lower<=upper;lower++){
    i=lower;
    while((x=i%10) != 0){
      if(x==find)
	count++;
      i=i/10;
 }
}
  return count;
    }

int main(){
  int lower,upper,find;
  printf("Enter lowerLimit upperLimit numberToFind:\n");
  scanf("%d %d %d",&lower,&upper,&find);
  /*printf("values taken\n");
    printf("lower:%d,upper:%d,number:%d\n",lower,upper,find);*/
  if(lower > upper || find > 10)
    printf("Invalid arguments: Either lower is greater than upper OR number is greater than 10\n");
  else{
    // printf("enter else");
      printf("No of occurences of %d in range %d - %d is : %d\n",find,lower,upper,nooccur(lower,upper,find));
    }
  return 0;
}
