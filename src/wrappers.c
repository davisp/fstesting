
#include <fcntl.h>
#include <sys/stat.h>

int open3(const char *path, int oflags, mode_t mode) {
  return open(path, oflags, mode);
}

int fcntl_int(int fd, int cmd, int arg) { return fcntl(fd, cmd, arg); }
