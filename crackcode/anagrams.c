#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define TRUE 1
#define FALSE 0

int anagram(char str1[], char str2[]){
    /*this one counts the number of each characters and matches*/
    int i = 0, ch;
    int letters[256];
    if(strlen(str1) != strlen(str2)){
        return FALSE;
    }
    for(i = 0; i < 256; i++){
        letters[i] = 0;
    }
    i = 0;
    while((ch = str1[i++])){
        letters[ch]++;
    }
    /*
    for(i = 0; i< 256; i++)
        printf("char %c, val = %d\n", i, letters[i]);
        */
    i = 0;
    while((ch = str2[i++])){
        if(letters[ch] == 0){
            return FALSE;
        }
        letters[ch]--;
    }
    for(i = 0; i < 256 && letters[i] == 0; i++);
    if(i == 256){
        return TRUE;
    }
    return FALSE;

}

void swap(void *arr, size_t sz, int i, int j){
    char *temp = (char *)malloc(sz);
    if(temp != NULL){
        memcpy(temp, (char *)arr+i, sz);
        memcpy((char *)arr+i, (char *)arr+j, sz);
        memcpy((char *)arr+j, temp, sz);
        free(temp);
    }
    else{
        perror("could not allocate memory\n");
    }
}

int partition(void *arr, size_t size, int left, int right, 
        int (*cmp)(void *, void *)){
    int i = left-1;
    int j = left;
    //void *pivot = arr[right];
    for(j = left; j < right; j++){
        if((cmp)(arr+(size*j), arr+(size*right)) < 0){
            i++;
            swap(arr, size, i, j);
        }
    }
    swap(arr, size, i+1, right);
    return i+1; 
}

void sort(void *arr, size_t size, int left, int right, 
        int (*cmp)(void *, void *)){
    int mid;
    if(left < right){
        mid = partition(arr, size, left, right, cmp);
        sort(arr, size, left, mid-1, cmp);
        sort(arr, size, mid+1, right, cmp);
    }
}

int anagram_sort(char s[], char t[]){
    /* this one just sorts the strings and compare */
    size_t slen = strlen(s);
    size_t tlen = strlen(t);
    if(slen != tlen)
        return FALSE;
    sort((void *)s, sizeof(char), 0, slen-1, 
            (int (*)(void *, void *))strcmp);
    sort((void *)t, sizeof(char), 0, tlen-1, 
           (int (*)(void *, void *))strcmp);
    if(strcmp(s, t) == 0)
        return TRUE;
    return FALSE;
}

int main(int argc, char *argv[]){
    if(argc < 2){
        printf("Invalid args, required 2 args\n");
        return 0;
    }
    
    if(anagram_sort(argv[1], argv[2])){
        printf(" Anagram \n");
        return 1;
        }
    printf("not anagrams\n");
    
    return 0;
}
