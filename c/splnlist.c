#include<stdio.h>
#include "list.h"

int main(){
    int i = 0;
    for(i = 1; i <= 9; i++){
        sp_insert(i);
    }
    sp_print(sp_head);
    for(i = 1; i < 4; i++ ){
        sp_delete(sp_search(i*i));
        sp_print(sp_head);
    }
    sp_reverse();
    sp_print(sp_head);
    return 0;
}
