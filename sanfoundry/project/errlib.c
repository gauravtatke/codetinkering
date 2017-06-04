#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>
#include "err.h"

#define MAXLINE 1024

FILE *logfile;

void err_dump(short fatal, short syserr, short log, const char *fmt, ...)
{
    char strbuff[MAXLINE];
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(strbuff, MAXLINE, fmt, ap);
    if (syserr) /* append error string also */
        snprintf(strbuff+strlen(strbuff), MAXLINE-strlen(strbuff), " : %s", strerror(errno));
    if (log) /* log in logfile */
        fprintf(logfile, "%s\n", strbuff);
    else
        fprintf(stderr, "%s\n", strbuff);
    va_end(ap);
    if (fatal)
        exit(EXIT_FAILURE);
}

/* int err(char *s) */
/* { */
/*     fprintf(stderr, "%s\n", s); */
/*     exit(EXIT_FAILURE); */
/* } */

/* 
 * Nonfatal error related to system call
 * print a message and return
 */
/* void err_ret(const char *fmt, ...) */
/* { */
/*     va_list ap; */
/*     va_start(ap, fmt); */
/*     error_dump(1, errno, fmt, ap); */
/*     va_end(ap); */
/* } */

/*
 * fatal error related to system call
 * print a message and terminate
 */
/* void err_sys(const char *fmt, ...) */
/* { */
/*     va_list ap; */
/*     va_start(ap, fmt); */
/*     error_dump(1, errno, fmt); */
/* } */

 
