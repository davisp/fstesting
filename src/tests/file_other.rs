/// UL1: Simple unlink / file deletion
#[test]
fn ul_01() {
    let mut path = crate::test_dir();
    path.push("unlink.txt");
    crate::create_file(&mut path, &[]);

    let err = unsafe { libc::access(path.c_str(), libc::F_OK) };
    assert_eq!(err, 0);

    let err = unsafe { libc::unlink(path.c_str()) };
    assert_eq!(err, 0);

    let err = unsafe { libc::access(path.c_str(), libc::F_OK) };
    assert_eq!(err, -1);
    assert_eq!(crate::errno(), libc::ENOENT);
}

// UL2: One hard link keeps file alive
#[test]
fn ul_02() {
    let mut path_src = crate::test_dir();
    let mut path_dst = path_src.clone();

    path_src.push("ul_02_src.txt");
    path_dst.push("ul_02_dst.txt");

    crate::create_file(&mut path_src, "Hello, World!".as_bytes());

    let err = unsafe { libc::link(path_src.c_str(), path_dst.c_str()) };
    assert_eq!(err, 0);

    let stat = crate::stat(&mut path_dst);
    assert_eq!(stat.st_nlink, 2);

    let err = unsafe { libc::unlink(path_src.c_str()) };
    assert_eq!(err, 0);

    let err = unsafe { libc::access(path_src.c_str(), libc::F_OK) };
    assert_eq!(err, -1);

    let stat = crate::stat(&mut path_dst);
    assert_eq!(stat.st_nlink, 1);

    assert_eq!(crate::read_file(&mut path_dst), "Hello, World!");

    let err = unsafe { libc::unlink(path_dst.c_str()) };
    assert_eq!(err, 0);

    let err = unsafe { libc::access(path_dst.c_str(), libc::F_OK) };
    assert_eq!(err, -1);
}

// UL3: Unlink on symlink target leaves symlink dangling
#[test]
fn ul_03() {
    let mut path_src = crate::test_dir();
    let mut path_dst = path_src.clone();

    path_src.push("ul_03_src.txt");
    path_dst.push("ul_03_dst.txt");

    crate::create_file(&mut path_src, "Hello, World!".as_bytes());

    let err = unsafe { libc::symlink(path_src.c_str(), path_dst.c_str()) };
    assert_eq!(err, 0);

    let stat = crate::stat(&mut path_src);
    assert_eq!(stat.st_nlink, 1);

    let err = unsafe { libc::unlink(path_src.c_str()) };
    assert_eq!(err, 0);

    let err = unsafe { libc::access(path_src.c_str(), libc::F_OK) };
    assert_eq!(err, -1);

    let fd = unsafe { libc::open(path_dst.c_str(), libc::O_RDONLY) };
    assert_eq!(fd, -1);
    assert_eq!(crate::errno(), libc::ENOENT);

    let stat = crate::lstat(&mut path_dst);
    assert_eq!(stat.st_nlink, 1);
}

// UL4: Unlink does not close/affect an open fd
#[test]
fn ul_04() {
    let mut path = crate::test_dir();
    path.push("ul_04.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let err = unsafe { libc::unlink(path.c_str()) };
    assert_eq!(err, 0);

    let err = unsafe { libc::access(path.c_str(), libc::F_OK) };
    assert_eq!(err, -1);

    let err = unsafe { libc::ftruncate(fd, 0) };
    assert_eq!(err, 0);

    let bytes = "Hello, Moon!";
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes().as_ptr() as *const libc::c_void,
            bytes.len(),
        )
    };
    assert_eq!(len, bytes.len() as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_SET) };
    assert_eq!(offset, 0);

    let mut bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::read(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
    };
    assert!(len > 0);
    bytes.resize(len as usize, 0);

    let value = String::from_utf8_lossy(&bytes);
    assert_eq!(value, "Hello, Moon!");

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert_eq!(fd, -1);
    assert_eq!(crate::errno(), libc::ENOENT);
}

// TR1: Simple truncate to empty file
#[test]
fn tr_01() {
    let mut path = crate::test_dir();
    path.push("tr_01.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());
    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let st = crate::stat(&mut path);
    assert_eq!(st.st_size, 13);

    let err = unsafe { libc::truncate(path.c_str(), 0) };
    assert_eq!(err, 0);

    let st = crate::stat(&mut path);
    assert_eq!(st.st_size, 0);
    assert_eq!(crate::read_file(&mut path), "");
}

// TR2: Truncate to extent fill
#[test]
fn tr_02() {
    let mut path = crate::test_dir();
    path.push("tr_02.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());
    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);
}

// TR3: Truncate on read-only file
#[test]
fn tr_03() {
    let mut path = crate::test_dir();
    path.push("tr_03.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());

    let err = unsafe { libc::truncate(path.c_str(), 0) };
    assert_eq!(err, -1);
    assert_eq!(crate::errno(), libc::EACCES);
}

// SQ1 (lseek): Simple move for reads
#[test]
fn sq_01() {
    let mut path = crate::test_dir();
    path.push("sq_01.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let offset = unsafe { libc::lseek(fd, 7, libc::SEEK_SET) };
    assert_eq!(offset, 7);

    let mut bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::read(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
    };
    assert!(len >= 0);
    bytes.resize(len as usize, 0);
    let value = String::from_utf8_lossy(&bytes).to_string();
    assert_eq!(value, "World!");

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

// SQ2: Error moving before 0
#[test]
fn sq_02() {
    let mut path = crate::test_dir();
    path.push("sq_02.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err = unsafe { libc::lseek(fd, -1, libc::SEEK_SET) };
    assert_eq!(err, -1);
    assert_eq!(crate::errno(), libc::EINVAL);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

// SQ3: seek beyond eof and write extends file
#[test]
fn sq_03() {
    let mut path = crate::test_dir();
    path.push("sq_03.txt");

    crate::create_file(&mut path, &[]);
    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let offset = unsafe { libc::lseek(fd, 1023, libc::SEEK_SET) };
    assert_eq!(offset, 1023);

    let mut bytes = vec![97u8; 1];
    let len = unsafe {
        libc::write(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
    };
    assert_eq!(len, bytes.len() as isize);

    let st = crate::stat(&mut path);
    assert_eq!(st.st_size, 1024);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

// SQ4: Show seek beyond eof does not allocate space
#[test]
fn sq_04() {
    let mut path = crate::test_dir();
    path.push("sq_04.txt");

    crate::create_file(&mut path, &[]);
    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let offset = unsafe { libc::lseek(fd, 1024 * 1024 - 1, libc::SEEK_SET) };
    assert_eq!(offset, 1024 * 1024 - 1);

    let mut bytes = vec![97u8; 1];
    let len = unsafe {
        libc::write(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
    };
    assert_eq!(len, bytes.len() as isize);

    let st = crate::stat(&mut path);
    let stfs = crate::statfs(&mut path);
    assert_eq!(st.st_size, 1024 * 1024);
    assert!(st.st_blocks * (stfs.f_bsize as i64) < (1024 * 1024));

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

// FALLOC1: Create file of zeros
#[cfg(target_os = "macos")]
#[test]
fn falloc_01() {
    let mut path = crate::test_dir();
    path.push("falloc_01.txt");

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR | libc::O_CREAT) };
    assert!(fd > 0);

    let allocated = unsafe {
        let mut fs: libc::fstore_t = std::mem::zeroed();
        fs.fst_flags = libc::F_ALLOCATECONTIG | libc::F_ALLOCATEALL;
        fs.fst_posmode = libc::F_PEOFPOSMODE;
        fs.fst_offset = 0;
        fs.fst_length = 1024 * 1024;
        fs.fst_bytesalloc = 0;

        let err = crate::fcntl_prealloc(fd, libc::F_PREALLOCATE, &mut fs);
        assert_eq!(err, 0);

        fs.fst_bytesalloc
    };

    assert!(allocated >= 1024 * 1024);
}

// FALLOC1: Create file of zeros
#[cfg(target_os = "linux")]
#[test]
fn falloc_01() {
    todo!();
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
#[test]
#[ignore = "falloc not supoprted"]
fn falloc_01() {}

// FALLOC2: Extend file with zeros
// FALLOC3: Punch hole - region is filled with zeroes
// FALLOC4: Collapse range - Remove middle of file
// FALLOC5: Zero range
// FALLOC6: Insert range

// FCNTL1: F_DUPFD
// FCNTL2: F_GETFD
// FCNTL3: F_SETFD
// FCNTL4: F_GETFL
// FCNTL5: F_SETFL
// FCNTL6: F_SETLK
// FCNTL7: F_SETLKW
// FCNTL8: F_GETLK

// FLOCK1: LOCK_SH
// FLOCK2: LOCK_EX
// FLOCK4: LOCK_SH with existing LOCK_EX
// FLOCK5: LOCK_EX with existing LOCK_SH
// FLOCK6: LOCK_EX with existing LOCK_EX

// FSYNC1: empty file
// FSYNC2: after write
// FSYNC3: every MiB writing multi-MiB file
// FSYNC4: separate thread every 100ms while main thread writes.

// FDATASYNC1: empty file
// FDATASYNC2: after write
// FDATASYNC3: every MiB writing multi-MiB file
// FDATASYNC4: separate thread every 100ms while main thread writes

// FADVISE1: NORMAL
// FADVISE2: SEQUENTIAL
// FADVISE3: RANDOM
// FADVISE4: NOREUSE
// FADVISE5: WILLNEED
// FADVISE6: DONTNEED

// CFR1: Copy whole file
// CFR2: Copy middle of file to new file
