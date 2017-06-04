#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <time.h>
#include <dirent.h>
#include <fcntl.h>
#include <string.h>
#include "tree.h"
#include "err.h"
#include "ipc.h"
#include "dir.h"

static char srch_path[MAX_PATH];
static time_t currtime; /*static variable used by getlatest() and restore()*/
static char ltst_dir[MAX_PATH];

int copy(const char *sfile, const char *dfile, mode_t md);
void set_stat(struct stat *bf, tptr nod);
char *basename(char *ch);

int getdir(char *ar[], char **s, char **d)
{
    struct stat buff;
    tptr nod;
    if ((**ar != '\0') && (stat(*ar, &buff) != -1) && ((buff.st_mode & S_IFMT) == S_IFDIR)){
        *s = *ar;
        nod = creat_nod(); //adding parent directory as root
        set_stat(&buff, nod);
        insert_nod(nod);
        if ((*++ar != '\0') && (stat(*ar, &buff) != -1) && ((buff.st_mode & S_IFMT) == S_IFDIR)){
            *d = *ar;
        }
        else
            *d = NULL;
      }
    else{
        *s = NULL;
        *d = NULL;
    }
    return 0;
}


void set_stat(struct stat *bf, tptr nod){
    nod->s_inode = bf->st_ino;
    nod->left = nod->right = nod->prnt = NULL;
    nod->s_mtime = bf->st_mtime;

}


int cpdir_t(const char *src, const char *dst, const unsigned short count){
    DIR *dfd;
    struct dirent *dp;
    struct stat buff;
    char s_path[MAX_PATH];
    char d_path[MAX_PATH];
    tptr nod, tmp;
    if ((dfd = opendir(src)) == NULL)
        err_dump(FATAL, SYSERR, LOG,"cpdir_t(): Could not opendir() %s\n", src);

    while ((dp = readdir(dfd)) != NULL){
        if (strcmp(dp->d_name,".") == 0 || strcmp(dp->d_name,"..") == 0)
            continue;//ignore the self and parent entry
        if (strlen(src)+strlen(dp->d_name)+2 > MAX_PATH){
            err_dump(!FATAL, !SYSERR, LOG,"cpdir_t(): Source name too long, skipping %s/%s\n", src, dp->d_name);
            continue;
        }
        if (strlen(dst)+strlen(dp->d_name)+2 > MAX_PATH){
            err_dump(!FATAL, !SYSERR, LOG,"cpdir_t(): Destination name too long, skipping %s/%s\n", dst, dp->d_name);
            continue;
        }
        
        sprintf(s_path,"%s%s%s",src,"/",dp->d_name);
        sprintf(d_path,"%s%s%s",dst,"/",dp->d_name);
        
        if (stat(s_path, &buff) == -1){
            err_dump(!FATAL, !SYSERR, LOG,"cpdir_t(): Could not state file OR dir %s\n", s_path);
            continue;
        }
        
        //if (count == 0)
        //free_t(root); //free-ing up all tree nodes

        if ((buff.st_mode & S_IFMT) == S_IFDIR){
            if (!mkdir(d_path, buff.st_mode))
                cpdir_t(s_path, d_path, count);
            else
                err_dump(FATAL, SYSERR, LOG,"cpdir_t(): Could not mkdir() %s\n", d_path);
        }
        else if (count == 0 || ((tmp = search_nod(buff.st_ino)) == NULL)){ //full backup
            nod = creat_nod(); //newly added file, adding to tree
            set_stat(&buff, nod);
            insert_nod(nod);
            copy(s_path, d_path, buff.st_mode);
        }

        else if (difftime(buff.st_mtime, tmp->s_mtime) > 0){
                tmp->s_mtime = buff.st_mtime; //updating the mtime to latest value
                copy(s_path, d_path, buff.st_mode);
        }
    }
    closedir(dfd);
    return 0;
}


int copy(const char *sfile, const char *dfile, mode_t md)
{
    short fdrd,fdwr,i;
    char ch[MAX_BYTE];
    if ((fdrd = open(sfile, O_RDONLY)) >= 0)
        if ((fdwr = open(dfile, O_WRONLY|O_CREAT|O_TRUNC, md)) >= 0)
            while((i = read(fdrd, &ch, MAX_BYTE)) > 0)
                write(fdwr, &ch, i);
        else
            err_dump(!FATAL, SYSERR, LOG,"copy(): Could not open() dest file %s, skipping it\n", dfile);
    else
            err_dump(!FATAL, SYSERR, LOG,"copy(): Could not open() source file %s, skipping it\n", sfile);
    close(fdrd);
    close(fdwr);
    return 0;
}


int do_copy(const char *src, const char *dest){
    char d_path[MAX_PATH];
    char s_path[MAX_PATH];
    char t_stamp[20];
    char bkp[32];
    unsigned short freq, count = 0;
    unsigned int len;
    time_t curtime;
    struct tm *loctime;
    shmptr sh_buff;
    
    sh_buff = getshm();//get shared memory
    semid = getsem(); //get and initialize semaphore
    
    len = strlen(src)-1;
    strcpy(s_path,src);
    if (s_path[len] == '/')
        s_path[len] = '\0';
    
    while((count = count%6) < 7){
        freq = get_nmin(sh_buff);
        curtime = time(NULL);
        loctime = localtime(&curtime);
        strftime(t_stamp, 20, "%F-%H-%M-%S",loctime);
        strcat(strcpy(bkp,basename(s_path)),"-");
        strcat(bkp,t_stamp);
        sprintf(d_path,"%s%s%s",dest,(dest[strlen(dest)-1]=='/')?"":"/",bkp);
        if (count == 0)
            free_t(root); //free-ing up all tree nodes
        //sprintf(d_path,"%s%s%s",dest,"/",bkp);
        if (!mkdir(d_path, 0755))
            cpdir_t(s_path, d_path, count);
        else
            err_dump(FATAL, SYSERR, LOG,"\ndo_copy(): Could not mkdir %s\n", d_path);
        count++;
        sleep(60*freq);
    }

    if (shmdt(sh_buff) == -1)
        err_dump(!FATAL, SYSERR, LOG, "\ndo_copy(): Error in detaching shared memory\n");

    if (semctl(semid, 0, IPC_RMID) == -1)
        err_dump(!FATAL, SYSERR, LOG, "\ndo_copy(): Error in deleting semaphore\n");

    return 0;
}


void list_dir(char *dir){
    DIR *dfd;
    struct dirent *dp;
    struct stat buff;
    char dir_path[MAX_PATH];
    if ((dfd = opendir(dir)) == NULL)
        err_dump(FATAL, SYSERR, LOG,"list_dir(): Could not opendir() %s\n", dir);

    printf("Top level directories are -\n");

    while ((dp = readdir(dfd)) != NULL){
        if (strcmp(dp->d_name,".") == 0 || strcmp(dp->d_name,"..") == 0)
            continue;//ignore the self and parent entry

        sprintf(dir_path,"%s%s%s",dir,(dir[strlen(dir)-1]=='/')?"":"/",dp->d_name);

        if (stat(dir_path, &buff) == -1){
            err_dump(!FATAL, SYSERR, LOG,"list_dir(): Could not stat() %s, skipping it\n", dir_path);
            continue;
        }
        
        if ((buff.st_mode & S_IFMT) == S_IFDIR)
            printf("%s\n",dir_path);
    }
}

char *get_tstamp(char *dirname){
    return dirname+4;
}

char *getlatest(DIR *dfd, char *srchdr, char *tm){
    struct dirent *dp;
    struct stat buff;
    char dir_path[MAX_PATH];
    time_t diftm = time(NULL);
    time_t df = -1;
    time_t dtm;
    static short found;
    ltst_dir[0] = '\0';
    
    while((dp = readdir(dfd)) != NULL){
        if (found)
            break; //coming out if found in prev iteration

        if (strcmp(dp->d_name,".") == 0 || strcmp(dp->d_name,"..") == 0)
            continue;//ignore the self and parent entry

        sprintf(dir_path,"%s%s%s",srchdr,"/",dp->d_name);

        if (stat(dir_path, &buff) == -1){

            err_dump(!FATAL, SYSERR, LOG,"getlatest(): Could not stat() %s, skipping it\n", dir_path);
            continue;
        }

        if ((buff.st_mode & S_IFMT) == S_IFDIR){
            if (tm != NULL){
                if (strcmp(get_tstamp(dp->d_name), tm) == 0){
                    strcpy(ltst_dir, dir_path);
                    found = 1;
                    break;
                }
            }
            else if ((df = difftime(currtime, buff.st_mtime)) > 0 &&  df < diftm){
                dtm = buff.st_mtime;
                diftm = df;
                strcpy(ltst_dir, dir_path);
            }
        }
    }

    currtime = dtm;
    rewinddir(dfd);
    if (ltst_dir[0] == '\0')
        return NULL;
    return ltst_dir;
 }

int emptychk(char *dr){
    /*If atleast one file present in dir hierarchy, function returns positive*/
    DIR *df;
    struct dirent *drnt;
    struct stat buff;
    char pt[MAX_PATH];
    static short isfile;
    
    if ((df = opendir(dr)) == NULL)
        err_dump(FATAL, SYSERR, LOG,"emptychk(): Could not opendir() %s\n", dr);
    
    while ((drnt = readdir(df)) != NULL){
        if (strcmp(drnt->d_name,".") == 0 || strcmp(drnt->d_name,"..") == 0)
            continue;//ignore the self and parent entry
        if (isfile)
            break;
        sprintf(pt,"%s%s%s",dr,"/",drnt->d_name);
        stat(pt, &buff);
        if ((buff.st_mode & S_IFMT) != S_IFDIR){
            /*any file is present, dir is not empty*/
            isfile = 1;
            break;
        }
        else /*if ((buff.st_mode & S_IFMT) == S_IFDIR)*/
            emptychk(pt);
    }
    closedir(df);
    return isfile;
}

char *searchdir(char *srchdr, char *tosrch){
    /* Searches tosrch in dst, returns path if found otherwise NULL */
    DIR *dr;
    char path[MAX_PATH];
    struct dirent *drnt;
    struct stat buff;
    static short done;
    //fprintf(stderr,"opendir - %s\n", srchdr);
    if ((dr = opendir(srchdr)) == NULL)
        err_dump(FATAL, SYSERR, LOG,"searchdir(): Could not opendir() source file %s\n", srchdr);
    while ((drnt = readdir(dr)) != NULL){
        if (strcmp(drnt->d_name,".") == 0 || strcmp(drnt->d_name,"..") == 0)
            continue;//ignore the self and parent entry

        if (done) /*to break from multiple levels of recursion*/
            break;

        sprintf(path,"%s%s%s",srchdr,"/",drnt->d_name);
        if (stat(path, &buff) == -1){
            err_dump(!FATAL, SYSERR, LOG,"searchdir(): Could not stat() %s, skipping it\n", path);
            continue;
        }

        if (strcmp(drnt->d_name,tosrch) == 0){
            if ((buff.st_mode & S_IFMT) == S_IFDIR){
                if ((done = emptychk(path))){//returns 0 if dir is empty i.e. without any reg file
                    //done = 1;
                    //fprintf(stderr,"Empty = %d\n", done);
                    strcpy(srch_path, path);
                    break;
                }
                else
                    //fprintf(stderr,"Empty dir\n");
                    continue;
                
            }
            done = 1;
            strcpy(srch_path, path);
            break;
        }
        
        if ((buff.st_mode & S_IFMT) == S_IFDIR)
            searchdir(path, tosrch);
    }

    closedir(dr);
    if (done)
        return srch_path;
    return NULL;
}


int restore(char *torest, char *dest, char *tstamp, shmptr shmbuf){
    DIR *dfd1;
    char *tmp1, *tmp2;
    char *srchin;
    struct stat buff;
    char d_path[MAX_PATH];
    short dn = 0;
    currtime = time(NULL);
    srchin = get_dest(shmbuf);
    if ((dfd1 = opendir(srchin)) == NULL)
        err_dump(FATAL, SYSERR, LOG,"restore(): Could not opendir() %s\n", srchin);

    while ((tmp1 = getlatest(dfd1, srchin, tstamp)) != NULL){
        if ((tmp2 = searchdir(tmp1, torest)) == NULL)
            continue; /* Not found in this directory */
        else{
            dn = 1;
            sprintf(d_path,"%s%s%s",dest,"/",basename(tmp2));
            stat(tmp2, &buff);
            if ((buff.st_mode & S_IFMT) == S_IFDIR){
                if (mkdir(d_path,buff.st_mode) == 0)
                    cpdir_t(tmp2, d_path, 0); /* copy whole dir*/
                else
                    err_dump(FATAL, SYSERR, LOG,"restore(): Could not mkdir() %s\n", d_path);
            }
            else
                copy(tmp2, d_path, buff.st_mode);
            break;
        }
    }
        
    closedir(dfd1);
    return dn;
}

char *basename(char *ch){
    int i = strlen(ch)-1;
    do
        i--;
    while (ch[i] != '/');
    return ch+i+1;
}

/* int main(int argc, char *argv[]){ */
/*     //int i = 0; */
/*     //list_dir(argv[1]); */
/*     //currtime = time(NULL); */
/*     if (restore(argv[1], argv[2], argv[3])) */
/*         printf("Restored\n"); */
/*     else */
/*         printf("Not found\n"); */
/*     return 0; */
/* } */
