#ifndef TREE_H_INCLUDED
#define TREE_H_INCLUDED

#include <sys/types.h>

struct mystat{
    ino_t s_inode;
    //time_t s_bkptime; 
    time_t s_mtime; /*stores the 
                      modification time 
                      since last backup */
    struct mystat *prnt;
    struct mystat *left;
    struct mystat *right;

} *root;
    
typedef struct mystat tnode;
typedef struct mystat *tptr;

tptr creat_nod(void);// creates a tree node and returns pointer to it
void insert_nod(tptr nod);// insert the node in the tree
tptr search_nod(ino_t inod);//search node based on inode number
void delet_nod(tptr nod);//deletes the node and frees the memory, returns pointer to parent
void free_t(tptr root); // frees up the memory of all nodes deleting the whole tree

#endif
