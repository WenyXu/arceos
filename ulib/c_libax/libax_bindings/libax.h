/* Generated by cbindgen and build.rs, DO NOT edit! */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/stat.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void ax_srand(uint32_t seed);

uint32_t ax_rand_u32(void);

int ax_print_str(const char *buf, size_t count);

void ax_panic(void);

/**
 * Open a file by `filename` and insert it into `FD_TABLE`, and return its
 * `FD_TABLE` index. Return `ENFILE` if file table overflow.
 */
int ax_open(const char *filename, int flags, mode_t mode);

/**
 * Close a file by `fd`.
 */
int ax_close(int fd);

/**
 * Seek the position of the file, return its position after seek.
 */
off_t ax_lseek(int fd, off_t offset, int whence);

/**
 * Read data from file by `fd`, return read size if success.
 */
ssize_t ax_read(int fd, void *buf, size_t count);

/**
 * Write data through `fd`, return written size if success.
 */
ssize_t ax_write(int fd, const void *buf, size_t count);

/**
 * Get file info by `path` and write to `buf`, return 0 if success.
 */
ssize_t ax_stat(const char *path, struct stat *buf);

/**
 * Get symbolic link info and write to `buf`, return 0 if success.
 */
ssize_t ax_lstat(const char *path, struct stat *buf);

/**
 * Get file info by `fd` and write to `buf`, return 0 if success.
 */
ssize_t ax_fstat(int fd, struct stat *buf);

/**
 * get the path of the current directory
 */
char *ax_getcwd(char *buf, size_t size);

/**
 * Allocate memory and return the memory address.
 *
 * Returns 0 on failure (the current implementation does not trigger an exception)
 */
void *ax_malloc(size_t size);

/**
 * Release memory.
 *
 * (WARNING) If the address to be released does not match the allocated address, an error should
 * occur, but it will NOT be checked out. This is due to the global allocator `Buddy_system`
 * (currently used) does not check the validity of address to be released.
 */
void ax_free(void *addr);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
