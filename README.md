# Filesystem Testing

This is a small repo to demonstrate running various fstests in GitHub Actions.

## APIs to Test

### File Properties

access()
chmod()
chown()

utime()

stat()

readlink()

setxattr()
getxattr()
listxattr()
removexattr()

### Files

create()
open()
close()
unlink()

read()
pread()
write()
pwrite()
truncate()
copy_file_range()

lseek()

flock()
flush()
fsync()
fallocate()

link()
symlink()
rename()

### Directories

mkdir()
opendir()
readdir()
closedir()
rmdir()
fsyncdir()

### Special

ioctl()
mknod()
poll()
statfs()
