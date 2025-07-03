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

## ToDo

- Check ls persistence of directories
- Check ls with more than 1000 directory entries

## Using mountpoint-s3 with minio

mount-s3 \
 --force-path-style \
 --allow-other \
 --allow-delete \
 --allow-overwrite \
 --endpoint-url http://localhost:9999/ \
 default-bucket \
 ./mountpoint

mount-s3 -f --force-path-style --allow-other --allow-delete --no-sign-request --endpoint-url http://localhost:8181/v4/files/     --x-tiledb-rest-api-key $TOKEN --prefix teamspace/ workspace mountpoint/ --allow-overwrite -d --incremental-upload
