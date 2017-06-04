#include<stdio.h>

/*Program to count number of occurence of digit in a
  specified range of integers */

int nooccur(int lower,int upper, int find){
  printf("inside nooccur\n");
  int i = lower,count = 0,x;
  for(i;i<=upper;i++){
    printf("inside for\n");
    while(x=i%10){
      printf("cursed to go round here forever\n");
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
  printf("values taken\n");
  printf("lower:%d,upper:%d,number:%d",lower,upper,find);
  if(lower > upper || find > 10){
    printf("Invalid arguments");}
  else{
      printf("enter else\n");
      printf("No of occurences of %d in range %d - %d is : %d\n",find,lower,upper,nooccur(lower,upper,find));
    }
  return 0;
}
