#include "ipc.h"
#include <string.h>
#include <errno.h>
#include "err.h"

int semid;
struct sembuf sbuf;
//union semun arg;

void P(){
    sbuf.sem_num = 0;
    sbuf.sem_op = -1;  /* set to allocate resource */
    sbuf.sem_flg = SEM_UNDO;
    
    if (semop(semid, &sbuf, 1) == -1)
        err_dump(FATAL, SYSERR, LOG,"P(): Could not semop()");
}

void V(){
    sbuf.sem_num = 0;
    sbuf.sem_op = 1;  /* set to allocate resource */
    sbuf.sem_flg = 0;
    
    if (semop(semid, &sbuf, 1) == -1)
        err_dump(FATAL, SYSERR, LOG,"V(): Could not semop()");
}


int get_nmin(shmptr shm){
    int nm;
    P();
    nm = shm->nmin;
    V();
    return nm;
}


void set_nmin(int nmin, shmptr shm){
    P();
    shm->nmin = nmin;
    V();
}


int get_count(shmptr shm){
    int cnt;
    P();
    cnt = shm->count;
    V();
    return cnt;
}


void set_count(int cnt, shmptr shm){
    P();
    shm->count = cnt;
    V();
}

char *get_dest(shmptr shm){
    char *ds;
    P();
    ds = shm->dest;
    V();
    return ds;
}


void set_dest(char *dst, shmptr shm){
    int i;
    i = strlen(dst)-1;
    P();
    strcpy(shm->dest,dst);
    if (shm->dest[i] == '/')
        shm->dest[i] = '\0';
    V();
}

int getsem(){/*returns one semaphore*/
    union semun arg;
    int sid;

    if ((sid = semget(KEY, 1, IPC_CREAT| IPC_EXCL| 0666)) >= 0){/*success*/
        arg.val = 1;
        semctl(sid, 0, SETVAL, arg);
    }
    else if (errno == EEXIST){/*already created*/
        if ((sid = semget(KEY, 1,0)) < 0) /*just get the semid*/
            return sid; /* error */
    }
    else /*error*/
        return sid;

    return sid;
}


shmptr getshm(){
    shmptr sh_buff;
    int shmid;
    void *shm_addr;
    if ((shmid = shmget(KEY,sizeof(shm_mem),0666|IPC_CREAT)) == -1)/*error*/
        //sys_err("shmget:");
        err_dump(FATAL, SYSERR, LOG,"getshm(): Could not shmget()");
    if ((shm_addr = shmat(shmid,NULL,0)) == (void *)-1)
        err_dump(FATAL, SYSERR, LOG,"getshm(): Could not shmget()");
        //sys_err("shmat:");
    sh_buff = (shmptr)shm_addr; /* casting void pointer to struct ptr */
    return sh_buff;
}
