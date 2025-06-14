
#include <fcntl.h>
#include <sys/stat.h>

int open3(const char *path, int oflags, mode_t mode) {
  return open(path, oflags, mode);
}

#ifdef __APPLE__

int fcntl_prealloc(int fd, int cmd, fstore_t *fs) { return fcntl(fd, cmd, fs); }

#endif
