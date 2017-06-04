#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <sys/wait.h>
#include "err.h"
#include "dir.h"
#include "ipc.h"

unsigned short nmin = 2, nm = 0, res = 0, lst = 0;

int parse_arg(int p_argc, char * const p_argv[]){
    char *endptr = '\0';
    extern int optind, opterr, optopt;
    extern char *optarg;
    int opt;
    opterr = 0;
    while ((opt = getopt(p_argc, p_argv, ":f:lr")) != -1){
            switch (opt){
            case 'f':
                nmin = strtol(optarg, &endptr, 10);
                if (*endptr != '\0' || nmin < 0)
                    err_dump(FATAL, !SYSERR, !LOG, "\nparse_arg(): Arg not positive integar\n \
 Usage: %s [-f nmin] [-l] [-r] src dst [timestamp]\n", p_argv[0]);
                else{
                    nm = 1;
                    res = 0;
                    lst = 0;
                }
                break;
            case 'l':
                lst = 1;
                res = 0;
                nm = 0;
                break;
            case 'r':
                res = 1;
                lst = 0;
                nm = 0;
                break;
            case ':':
                err_dump(FATAL, !SYSERR, !LOG, "\nparse_arg(): Option arg missing\n \
Usage: %s [-f nmin] [-l] [-r] src dst [timestamp]\n", p_argv[0]);
                break;
            default: /* '?' */
                err_dump(FATAL, !SYSERR, !LOG, "\nparse_arg(): Invalid option - options to use [flr]\n \
Usage: %s [-f nmin] [-l] [-r] src dst [timestamp]\n", p_argv[0]);
                break;
            }
        }
    return optind;
}

int main(int argc, char *argv[]){
    char *src = '\0'; 
    char *dst = '\0';
    char *t_stamp = NULL;
    int pid;
    short m_optind;
    //int stat = 1;
    shmptr sh_buff;
    char *name = argv[0];
    logfile = fopen("./log_lbackup", "a+");

    if (argc < 2 || argc > 6)
        err_dump(FATAL, !SYSERR, !LOG, "\nmain(): Invalid no. of arguements\n \
Usage: %s [-f nmin] [-l] [-r] src dst [timestamp]\n", argv[0]);
    else
        m_optind = parse_arg(argc, argv);
    argc -= m_optind;
    argv += m_optind;

    sh_buff = getshm(); //get the shared memory
    
    /*semid is declared globally in ipclib.c*/
    semid = getsem(); //get and initialize semaphore
    
    if (nm || !(lst || res)){
        if (argc >= 2){
            getdir(argv, &src, &dst);
            if (src && *src != '\0' && dst && *dst != '\0'){ //src-dst arg given
                set_dest(dst, sh_buff);
                set_nmin(nmin, sh_buff);
                pid = fork();
                if (!pid){ /*child process*/
                    do_copy(src, dst);
                }
                if (pid == -1)
                    err_dump(FATAL, SYSERR, !LOG, "\nmain(): Could not fork process\n");
                if (pid > 0 ){/*Parent code*/
                    exit(EXIT_SUCCESS);
                    //wait(&stat);
                }
            }
            else
                err_dump(FATAL, !SYSERR, !LOG, "\nmain(): Invalid src or dst dir for [-f] option\n \
Usage: %s [-f nmin] [-l] [-r] src dst [timestamp]\n", name);
        }
        else if (argc == 0)
            set_nmin(nmin, sh_buff);
        else
            err_dump(FATAL, !SYSERR, !LOG, "\nmain(): Missing dest arg for [-f] option\n \
Usage: %s [-f nmin] [-l] [-r] src dst [timestamp]\n", name);
    }
    else if (lst)
        list_dir(get_dest(sh_buff));
    
    else if (res){
        if (argc >= 2){
            src = *argv;
            dst = *++argv;
            t_stamp = NULL;
            argc -= 2;
        }
        else
            err_dump(FATAL, !SYSERR, !LOG,"\nmain(): Either source file/dir or destination is missing\n \
Usage: %s [-f nmin] [-l] [-r] src dst [timestamp]\n", name);
        if (argc) //timestamp present
            t_stamp = *++argv;
        if (restore(src, dst, t_stamp, sh_buff) == 0)
            err_dump(FATAL, !SYSERR, !LOG, "\nmain(): Could not restore\n");
    }
    else /* no options are set*/
        err_dump(FATAL, !SYSERR, !LOG,"main(): Either source file/dir or destination is missing\n \
Usage: %s [-f nmin] [-l] [-r] src dst [timestamp]\n", name);
    
    if (shmdt(sh_buff) == -1)
        err_dump(!FATAL, SYSERR, LOG, "\nmain(): Error in detaching shared memory\n");

    /* if (semctl(semid, 0, IPC_RMID) == -1) */
    /*     err_dump(!FATAL, SYSERR, LOG, "\nmain(): Error in deleting semaphore\n"); */

    return 0;
}
