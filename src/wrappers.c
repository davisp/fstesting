
#include <fcntl.h>
#include <sys/stat.h>

int open3(const char *path, int oflags, mode_t mode) {
  return open(path, oflags, mode);
}

int fcntl_int(int fd, int cmd, int arg) { return fcntl(fd, cmd, arg); }

#ifdef __APPLE__

int fcntl_prealloc(int fd, int cmd, fstore_t *fs) { return fcntl(fd, cmd, fs); }

int fcntl_punchhole(int fd, int cmd, fpunchhole_t *fph) {
  return fcntl(fd, cmd, fph);
}

#endif
