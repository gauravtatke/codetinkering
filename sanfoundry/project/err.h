#ifndef ERR_H_INCLUDED
#define ERR_H_INCLUDED

#include<stdio.h>
#include<stdlib.h>
#include<error.h>

#define FATAL 1
#define SYSERR 1
#define LOG 1

#define sys_err(msg) \
    do {perror(msg); exit(EXIT_FAILURE);} while (0)

#define err(msg) \
    do {printf(msg); exit(EXIT_FAILURE);} while (0)

extern FILE *logfile;

void err_dump(short fatal, short syserr, short log, const char *fmt, ...);

#endif
