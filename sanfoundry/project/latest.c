#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <time.h>
#include <dirent.h>
#include <fcntl.h>
#include <string.h>
#include <stdio.h>

#define MAX_PATH 512

char *get_tstamp(char *dirname){
    //char *i;
    return dirname+4;
}

static time_t currtime; /*static variable used by getlatest() and restore()*/
char ltst_dir[MAX_PATH];

char *getlatest(DIR *dfd, char *dst, char *tm){
    //DIR *dfd;
    struct dirent *dp;
    struct stat buff;
    char dir_path[MAX_PATH];
    time_t diftm = time(NULL);
    time_t df = -1, dtm;
    static short found;
    ltst_dir[0] = '\0';
    
    while((dp = readdir(dfd)) != NULL){
        if (found)
            break;

        if (strcmp(dp->d_name,".") == 0 || strcmp(dp->d_name,"..") == 0)
            continue;//ignore the self and parent entry

        sprintf(dir_path,"%s%s%s",dst,(dst[strlen(dst)-1]=='/')?"":"/",dp->d_name);

        if (stat(dir_path, &buff) == -1){
            fprintf(stderr,"could not stat the file/dir, skipping it: %s\n", dir_path);
            continue;
        }
        //printf("curr dir = %s\n", dir_path);
        if ((buff.st_mode & S_IFMT) == S_IFDIR){
            if (tm != NULL){
                if (strcmp(get_tstamp(dp->d_name), tm) == 0){
                    //ltst_dir = dir_path;
                    strcpy(ltst_dir, dir_path);
                    //printf("final dir inside loop -> %s\n", ltst_dir);
                    found = 1;
                    break;
                }
            }
            else if ((df = difftime(currtime, buff.st_mtime)) > 0 &&  df < diftm){
                dtm = buff.st_mtime;
                diftm = df;
                //ltst_dir = dir_path;
                 strcpy(ltst_dir, dir_path);
                 //printf("so far latest dir = %s, df = %f, diftm = %f\n", ltst_dir, (double)df, (double)diftm);
            }
            //else
                //printf("file = %s, df = %f, diftm = %f\n",dir_path, (double)df, (double)diftm);
        }
    }
    
    //printf("final dir -> %s,  %f\n", ltst_dir, (double)df);
    /* if (df < 0)/\*reached end, no file latest than this*\/ */
    /*     return NULL; */

    currtime = dtm;
    rewinddir(dfd);
    if (ltst_dir[0] == '\0')
        return NULL;
    return ltst_dir;
    //return t;
 }

int main(int argc, char *argv[]){
    currtime = time(NULL);
    DIR *dfd;
    char *tmp1;
    if ((dfd = opendir(argv[1])) == NULL)
        perror("opendir::");
    //tmp1 = getlatest(dfd, argv[1],NULL);
    while ((tmp1 = getlatest(dfd, argv[1],argv[2])) != NULL)
        printf("latest => %s\n", tmp1);
    closedir(dfd);
    return 0;
}

