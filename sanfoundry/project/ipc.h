#ifndef IPC_H_INCLUDED
#define IPC_H_INCLUDED

#include <sys/ipc.h>
#include <sys/shm.h>
#include <sys/sem.h>
#include <sys/types.h>

#define KEY 4375

struct shared_mem{
    int nmin;
    int count;
    char dest[512];
};

typedef struct shared_mem shm_mem;
typedef struct shared_mem *shmptr;

shmptr getshm();
int get_nmin(shmptr shm);
void set_nmin(int nmin, shmptr shm);
int get_count(shmptr shm);
void set_count(int cnt, shmptr shm);
char *get_dest(shmptr shm);
void set_dest(char *dst, shmptr shm);

union semun{
    int val;
    struct semid_ds *buf;
    ushort *array;
};

extern struct sembuf sbuf;
extern int semid;

void P();
void V();
int getsem();

#endif
