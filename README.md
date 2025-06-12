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

## Using mountpoint-s3 with minio

mount-s3 \
 --force-path-style \
 --allow-other \
 --allow-delete \
 --allow-overwrite \
 --endpoint-url http://localhost:9999/ \
 default-bucket \
 ./mountpoint
