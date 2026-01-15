#ifndef _LIB_H
#define _LIB_H

#include <stddef.h>

typedef long isize;

#define SYSCALL_OPEN 56
#define SYSCALL_CLOSE 57
#define SYSCALL_READ 63
#define SYSCALL_WRITE 64
#define SYSCALL_EXIT 93

isize syscall(size_t id, size_t arg0, size_t arg1, size_t arg2);
isize open(const char *path, int flags);
isize close(int fd);
isize read(int fd, void *buf, size_t count);
isize write(int fd, const void *buf, size_t count);
void exit(int code);

#endif
