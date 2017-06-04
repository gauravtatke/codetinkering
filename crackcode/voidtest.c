#include <stdio.h>

void swap(void **s, void **t){
    printf("add of s = %p, t = %p\n", &s, &t);
    printf("s = %p, t = %p\n", s, t);
    printf("val at s = %d, t = %d\n", *((int *)s), *((int *)t));
    void *temp = *s;
    *s = *t;
    *t = temp;
}

void swap2(int **s, int **t){
    printf("add of s = %p, t = %p\n", &s, &t);
    printf("s = %p, t = %p\n", s, t);
    printf("val at s = %d, t = %d\n", *((int *)s), *((int *)t));
    void *temp = *s;
    *s = *t;
    *t = temp;
}

int main(){
    int i = 24;
    int j = 43;
    void *p = &i;
    void *q = &j;
    int *pi = &i;
    int *pj = &j; 
    printf("add of i = %p, j = %p\n", &i, &j);
    printf("p = %p, q = %p\n", p, q);
    printf("add of p = %p, q = %p\n", &p, &q);
    printf("val at p = %d, q = %d\n", *((int *)p), *((int *)q));
    swap(p, q);
    swap2(pi, pj);
    printf("i = %d, j = %d\n", i, j);
    printf("val at p after = %d, q = %d\n", *((int *)p), *((int *)q));
    return 0;
}
