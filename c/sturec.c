#include<stdio.h>
/* This program insert registration number, name, age
   and marks of students in a file and searches for record */
int age,mark1,mark2,mark3;
char regno[10],name[15];

void setdata(int nstu){
  int i;
  FILE *fp = fopen("sturecord","a");
  for(i=0;i<nstu;i++){
    printf("Enter registration number:");
    scanf("%s",regno);
    printf("Enter name:");
    scanf("%s",name);
    printf("Enter age:");
    scanf("%d",&age);
    printf("Enter mark 1:");
    scanf("%d",&mark1);
    printf("Enter mark 2:");
    scanf("%d",&mark2);
    printf("Enter mark 3:");
    scanf("%d",&mark3);
    fprintf(fp,"%s\t%s\t%d\t%d\t%d\t%d\n",regno,name,age,mark1,mark2,mark3);
  }
}


int searchdata(char *rgno){
  int found=0;
  FILE *fp = fopen("sturecord","r");
  while(fscanf(fp,"%s\t%s\t%d\t%d\t%d\t%d\n",regno,name,&age,&mark1,&mark2,&mark3) != EOF){
    if(strcmp(regno,rgno) == 0){
      printf("Registration No : %s, Name : %s, Age : %d\n",regno,name,age);
      printf("Mark 1 : %d, Mark 2 : %d, Mark 3 : %d\n",mark1,mark2,mark3);
      found++;
    }
  }
  if(found == 0)
    printf("Sorry: No matching records found\n");
  return 0;
}

int main(){
  int nstu;
  char ch;
  char srch[10];
  int i =0;
  printf("Enter Choice:insert-i,search-s,quit-q :: ");
  while((ch = getchar()) != 'q'){
    switch(ch){
    case 'i':
      printf("Enter number of students : ");
      scanf("%d",&nstu);
      setdata(nstu);
      break;
    case 's':
      printf("Enter registration no : ");
      scanf("%s",srch);
      searchdata(srch);
      break;
    default:
      printf("Invalid option,try again!!\n");
      break;
    }
    while((ch=getchar()) != '\n'); /*to flush stdin, otherwise it uses ENTER(\n) as
                                     next character and executes above while loop*/
    printf("Enter Choice [isq]: ");
  }
}
