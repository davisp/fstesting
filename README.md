# Filesystem Testing

This is a small repo to demonstrate running various fstests in GitHub Actions.

## APIs to Test

### File Properties

- access()
- chmod()
- chown()
- utime()
- stat()
- readlink()
- setxattr()
- getxattr()
- listxattr()
- removexattr()
- link()
- symlink()
- rename()

### Files

- create()
- open()
- close()
- unlink()
- read()
- pread()
- write()
- pwrite()
- truncate()
- copy_file_range()
- lseek()
- fadvise
- fallocate()
- fcntl
- fdatasync()
- flock()
- flush()
- fsync()

### Directories

- mkdir()
- opendir()
- readdir()
- closedir()
- rmdir()
- fsyncdir()

### Special

- statfs()

### ToDo

- error opening directory without execute permission
- dangling symbolic link
- open through symbolic link
- O_APPEND + truncate + pread
- O_APPEND + pwrite - check lseek behavior
- observe writes through separate fds
- O_CREAT | O_EXCL
