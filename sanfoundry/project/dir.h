#ifndef DIR_H_INCLUDED
#define DIR_H_INCLUDED

#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <dirent.h>
#include "tree.h"
#include "ipc.h"

#define MAX_PATH 512 //Max path length
#define MAX_BYTE 4096 //max bytes transfer in copy

int getdir(char *ar[], char **s, char **d);
//int isdir(char *pt);
int copy(const char *sfile, const char *dfile, mode_t md);
void set_stat(struct stat *bf, tptr nod);
int cpdir_t(const char *src, const char *dst, const unsigned short count); //count = 0 mean full backup, otherwise based on last modified time
int do_copy(const char *src, const char *dst);
void list_dir(char *dir);
int restore(char *torest, char *dest, char *tstamp, shmptr shmbuf);
char *basename(char *ch);
char *searchdir(char *srchdr, char *tosrch);
char *get_tstamp(char *dirname);
char *getlatest(DIR *dfd, char *srchdr, char *tm);
int emptychk(char *dr);

#endif
