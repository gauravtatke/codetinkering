#include <stdlib.h>
#include <stdio.h>
#include "tree.h"

size_t tsize = sizeof(tnode);

tptr creat_nod(){
    tptr tmp = (tptr)malloc(tsize);
    return tmp;
}


void insert_nod(tptr nod){
    tptr tmp = root;
    if (tmp == NULL){ //empty tree
        root = nod;
    }
    else{
        while(1){
            if (nod->s_inode < tmp->s_inode){
                if (tmp->left == NULL){
                    tmp->left = nod;
                    break;
                }
                tmp = tmp->left;
            }
            else if (tmp->right == NULL){
                tmp->right = nod;
                break;
            }
            else
                tmp = tmp->right;
        }
    }
    nod->prnt = tmp;
    nod->left = nod->right = NULL;
}


tptr search_nod(ino_t inod){
    tptr tmp = root;
    while(tmp != NULL && tmp->s_inode != inod){
        if (inod < tmp->s_inode)
            tmp = tmp->left;
        else
            tmp = tmp->right;
    }
    return tmp;
}


void delete_nod(tptr nod){
    if (nod->prnt != NULL){//if its root, delete directly
        if ((nod->prnt)->left == nod)
            (nod->prnt)->left = NULL;
        else
            (nod->prnt)->right = NULL;
    }
    free(nod);
}


void free_t(tptr nod){
    if (nod != NULL){
        if (nod->left == NULL && nod->right == NULL)
            delete_nod(nod);
        else{
            free_t(nod->left);
            free_t(nod->right);
            free_t(nod);
        }
    }
}
