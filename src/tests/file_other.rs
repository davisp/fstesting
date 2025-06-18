use std::sync::mpsc;

/// unlink_01: Simple unlink / file deletion
#[test]
fn unlink_01() {
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

/// unlink_02: One hard link keeps file alive
#[test]
fn unlink_02() {
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

/// unlink_03: Unlink on symlink target leaves symlink dangling
#[test]
fn unlink_03() {
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

/// unlink_04: Unlink does not close/affect an open fd
#[test]
fn unlink_04() {
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

/// symlink_01: Open file through symlink
#[test]
fn symlink_01() {
    let mut path_src = crate::test_dir();
    let mut path_dst = path_src.clone();

    path_src.push("symlink_03_src.txt");
    path_dst.push("symlink_03_dst.txt");

    crate::create_file(&mut path_src, "Hello, World!".as_bytes());

    let err = unsafe {
        libc::symlink(
            c"symlink_03_src.txt".as_ptr() as *const libc::c_char,
            path_dst.c_str(),
        )
    };
    assert_eq!(err, 0);

    let stat = crate::stat(&mut path_src);
    assert_eq!(stat.st_nlink, 1);

    let fd = unsafe { libc::open(path_dst.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let mut bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::read(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
    };
    assert!(len > 0);
    bytes.resize(len as usize, 0);
    let value = String::from_utf8_lossy(&bytes).to_string();
    assert_eq!(value, "Hello, World!");

    let stat = crate::lstat(&mut path_dst);
    assert_eq!(stat.st_nlink, 1);
}

/// truncate_01: Simple truncate to empty file
#[test]
fn truncate_01() {
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

/// truncate_02: Truncate to extent fill
#[test]
fn truncate_02() {
    let mut path = crate::test_dir();
    path.push("tr_02.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());
    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);
}

/// truncate_03: Truncate on read-only file
#[test]
fn truncate_03() {
    let mut path = crate::test_dir();
    path.push("tr_03.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());

    let err = unsafe { libc::truncate(path.c_str(), 0) };
    assert_eq!(err, -1);
    assert_eq!(crate::errno(), libc::EACCES);
}

/// seek_01: Simple move for reads
#[test]
fn seek_01() {
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

/// seek_02: Error moving before 0
#[test]
fn seek_02() {
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

/// seek_03: seek beyond eof and write extends file
#[test]
fn seek_03() {
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

/// seek_04: Show seek beyond eof does not allocate space
#[test]
fn seek_04() {
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

/// falloc_01: Allocate file space for empty file
#[cfg(target_os = "linux")]
#[test]
fn falloc_01() {
    let mut path = crate::test_dir();
    path.push("falloc_01.txt");

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR | libc::O_CREAT) };
    assert!(fd > 0);

    let err = unsafe { libc::posix_fallocate(fd, 0, 1024 * 1024) };
    assert_eq!(err, 0);

    let st = crate::stat(&mut path);
    let stfs = crate::statfs(&mut path);
    assert!(st.st_blocks * (stfs.f_bsize as i64) >= (1024 * 1024));

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// falloc_02: Extend existing non-empty file
#[cfg(target_os = "linux")]
#[test]
fn falloc_02() {
    let mut path = crate::test_dir();
    path.push("falloc_02.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());
    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR | libc::O_CREAT) };
    assert!(fd > 0);

    let err = unsafe { libc::posix_fallocate(fd, 12, 512) };
    assert_eq!(err, 0);

    let st = crate::stat(&mut path);
    let stfs = crate::statfs(&mut path);
    assert!(st.st_blocks * (stfs.f_bsize as i64) >= 525);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// Skipping these for now.
/// falloc_03: Punch hole - region is filled with zeroes
/// falloc_04: Collapse range - Remove middle of file
/// falloc_05: Zero range
/// falloc_06: Insert range

/// fcntl_01: F_DUPFD
#[test]
fn fcntl_01() {
    let mut path = crate::test_dir();
    path.push("fcntl_01.txt");

    crate::create_file(&mut path, "Hello, World!".as_bytes());

    let fd1 = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd1 > 0);

    let fd2 = unsafe { libc::fcntl(fd1, libc::F_DUPFD) };
    assert!(fd2 > 0);

    let err = unsafe { libc::close(fd1) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd2) };
    assert_eq!(err, 0);
}

/// fcntl_02: F_GETFD
#[test]
fn fcntl_02() {
    let mut path = crate::test_dir();
    path.push("fcntl_02.txt");

    crate::create_file(&mut path, &[]);

    let fd =
        unsafe { libc::open(path.c_str(), libc::O_RDONLY | libc::O_CLOEXEC) };
    assert!(fd > 0);

    let flags = unsafe { crate::fcntl_int(fd, libc::F_GETFD, 0) };
    assert_eq!(flags & libc::FD_CLOEXEC, libc::FD_CLOEXEC);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fcntl_03: F_SETFD
#[test]
fn fcntl_03() {
    let mut path = crate::test_dir();
    path.push("fcntl_02.txt");

    crate::create_file(&mut path, &[]);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let flags = unsafe { crate::fcntl_int(fd, libc::F_GETFD, 0) };
    assert_ne!(flags & libc::FD_CLOEXEC, libc::FD_CLOEXEC);

    let err = unsafe { crate::fcntl_int(fd, libc::F_SETFD, libc::FD_CLOEXEC) };
    assert_eq!(err, 0);

    let flags = unsafe { crate::fcntl_int(fd, libc::F_GETFD, 0) };
    assert_eq!(flags & libc::FD_CLOEXEC, libc::FD_CLOEXEC);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fcntl_04: F_GETFL
#[test]
fn fcntl_04() {
    let mut path = crate::test_dir();
    path.push("fcntl_04.txt");

    crate::create_file(&mut path, &[]);

    let fd =
        unsafe { libc::open(path.c_str(), libc::O_RDONLY | libc::O_NONBLOCK) };
    assert!(fd > 0);

    let flags = unsafe { crate::fcntl_int(fd, libc::F_GETFL, 0) };
    assert_eq!(flags & libc::O_NONBLOCK, libc::O_NONBLOCK);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fcntl_05: F_SETFL
#[test]
fn fcntl_05() {
    let mut path = crate::test_dir();
    path.push("fcntl_05.txt");

    crate::create_file(&mut path, &[]);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let flags = unsafe { crate::fcntl_int(fd, libc::F_GETFL, 0) };
    assert_ne!(flags & libc::O_NONBLOCK, libc::O_NONBLOCK);

    let err = unsafe { crate::fcntl_int(fd, libc::F_SETFL, libc::O_NONBLOCK) };
    assert_eq!(err, 0);

    let flags = unsafe { crate::fcntl_int(fd, libc::F_GETFL, 0) };
    assert_eq!(flags & libc::O_NONBLOCK, libc::O_NONBLOCK);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// flock_01: LOCK_SH
#[test]
fn flock_01() {
    let mut path = crate::test_dir();
    path.push("flock_01.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_SH) };
    assert_eq!(err, 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_UN) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// flock_02: LOCK_EX
#[test]
fn flock_02() {
    let mut path = crate::test_dir();
    path.push("flock_02.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_EX) };
    assert_eq!(err, 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_UN) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// flock_03: LOCK_SH with existing LOCK_EX
#[test]
fn flock_03() {
    let mut path = crate::test_dir();
    path.push("flock_03.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_EX) };
    assert_eq!(err, 0);

    let mut thr_path = path.clone();
    let t = std::thread::spawn(move || {
        let thr_fd = unsafe { libc::open(thr_path.c_str(), libc::O_RDWR) };
        assert!(thr_fd > 0);

        let err = unsafe { libc::flock(thr_fd, libc::LOCK_SH | libc::LOCK_NB) };
        assert_eq!(err, -1);
        assert_eq!(crate::errno(), libc::EWOULDBLOCK);
    });

    t.join().unwrap();

    let err = unsafe { libc::flock(fd, libc::LOCK_UN) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// flock_04: LOCK_EX with existing LOCK_SH
#[test]
fn flock_04() {
    let mut path = crate::test_dir();
    path.push("flock_04.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_SH) };
    assert_eq!(err, 0);

    let mut thr_path = path.clone();
    let t = std::thread::spawn(move || {
        let thr_fd = unsafe { libc::open(thr_path.c_str(), libc::O_RDWR) };
        assert!(thr_fd > 0);

        let err = unsafe { libc::flock(thr_fd, libc::LOCK_EX | libc::LOCK_NB) };
        assert_eq!(err, -1);
        assert_eq!(crate::errno(), libc::EWOULDBLOCK);

        let err = unsafe { libc::close(thr_fd) };
        assert_eq!(err, 0);
    });

    t.join().unwrap();

    let err = unsafe { libc::flock(fd, libc::LOCK_UN) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// flock_05: LOCK_EX with existing LOCK_EX
#[test]
fn flock_05() {
    let mut path = crate::test_dir();
    path.push("flock_05.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_EX) };
    assert_eq!(err, 0);

    let mut thr_path = path.clone();
    let t = std::thread::spawn(move || {
        let thr_fd = unsafe { libc::open(thr_path.c_str(), libc::O_RDWR) };
        assert!(thr_fd > 0);

        let err = unsafe { libc::flock(thr_fd, libc::LOCK_EX | libc::LOCK_NB) };
        assert_eq!(err, -1);
        assert_eq!(crate::errno(), libc::EWOULDBLOCK);

        let err = unsafe { libc::close(thr_fd) };
        assert_eq!(err, 0);
    });

    t.join().unwrap();

    let err = unsafe { libc::flock(fd, libc::LOCK_UN) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// flock_06: LOCK_SH to LOCK_EX
#[test]
fn flock_06() {
    let mut path = crate::test_dir();
    path.push("flock_06.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_SH) };
    assert_eq!(err, 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_EX) };
    assert_eq!(err, 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_UN) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// flock_07: LOCK_EX to LOCK_SH
#[test]
fn flock_07() {
    let mut path = crate::test_dir();
    path.push("flock_07.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_EX) };
    assert_eq!(err, 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_SH) };
    assert_eq!(err, 0);

    let err = unsafe { libc::flock(fd, libc::LOCK_UN) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fsync_01: empty file
#[test]
fn fsync_01() {
    let mut path = crate::test_dir();
    path.push("fsync_01.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let err = unsafe { libc::fsync(fd) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fsync_02: after write
#[test]
fn fsync_02() {
    let mut path = crate::test_dir();
    path.push("fsync_02.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::write(fd, bytes.as_ptr() as *const libc::c_void, bytes.len())
    };
    assert_eq!(len, 1024);

    let err = unsafe { libc::fsync(fd) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fsync_03: every MiB writing multi-MiB file
#[test]
fn fsync_03() {
    let mut path = crate::test_dir();
    path.push("fsync_03.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let bytes = vec![127u8; 1024 * 1024];
    for _ in 0..15 {
        let len = unsafe {
            libc::write(fd, bytes.as_ptr() as *const libc::c_void, bytes.len())
        };
        assert_eq!(len, 1024 * 1024);

        let err = unsafe { libc::fsync(fd) };
        assert_eq!(err, 0);
    }

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fsync_04: fsync from separate thread
#[test]
fn fsync_04() {
    let mut path = crate::test_dir();
    path.push("fsync_04.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let (tx, rx) = mpsc::channel();

    let mut thr_path = path.clone();
    let t = std::thread::spawn(move || {
        let thr_fd = unsafe { libc::open(thr_path.c_str(), libc::O_RDWR) };
        assert!(thr_fd > 0);

        loop {
            let do_fsync = rx.recv().unwrap();
            if do_fsync {
                let err = unsafe { libc::fsync(thr_fd) };
                assert_eq!(err, 0);
            } else {
                break;
            }
        }

        let err = unsafe { libc::close(thr_fd) };
        assert_eq!(err, 0);
    });

    let bytes = vec![127u8; 1024 * 1024];
    for _ in 0..15 {
        let len = unsafe {
            libc::write(fd, bytes.as_ptr() as *const libc::c_void, bytes.len())
        };
        assert_eq!(len, 1024 * 1024);

        tx.send(true).unwrap();
    }

    tx.send(false).unwrap();
    t.join().unwrap();

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fsync_05: fsync on read-only file
#[test]
fn fsync_05() {
    let mut path = crate::test_dir();
    path.push("fsync_01.txt");

    crate::create_file(&mut path, &[]);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err = unsafe { libc::fsync(fd) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fadvise_01: NORMAL
#[cfg(target_os = "linux")]
#[test]
fn fadvise_01() {
    let mut path = crate::test_dir();
    path.push("fadvise_01.txt");

    let data = vec![0u8; 1024];
    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err =
        unsafe { libc::posix_fadvise(fd, 0, 1024, libc::POSIX_FADV_NORMAL) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fadvise_02: SEQUENTIAL
#[cfg(target_os = "linux")]
#[test]
fn fadvise_02() {
    let mut path = crate::test_dir();
    path.push("fadvise_02.txt");

    let data = vec![0u8; 1024];
    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err = unsafe {
        libc::posix_fadvise(fd, 0, 1024, libc::POSIX_FADV_SEQUENTIAL)
    };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fadvise_03: RANDOM
#[cfg(target_os = "linux")]
#[test]
fn fadvise_03() {
    let mut path = crate::test_dir();
    path.push("fadvise_03.txt");

    let data = vec![0u8; 1024];
    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err =
        unsafe { libc::posix_fadvise(fd, 0, 1024, libc::POSIX_FADV_RANDOM) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fadvise_04: NOREUSE
#[cfg(target_os = "linux")]
#[test]
fn fadvise_04() {
    let mut path = crate::test_dir();
    path.push("fadvise_04.txt");

    let data = vec![0u8; 1024];
    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err =
        unsafe { libc::posix_fadvise(fd, 0, 1024, libc::POSIX_FADV_NOREUSE) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fadvise_05: WILLNEED
#[cfg(target_os = "linux")]
#[test]
fn fadvise_05() {
    let mut path = crate::test_dir();
    path.push("fadvise_05.txt");

    let data = vec![0u8; 1024];
    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err =
        unsafe { libc::posix_fadvise(fd, 0, 1024, libc::POSIX_FADV_WILLNEED) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// fadvise_06: DONTNEED
#[cfg(target_os = "linux")]
#[test]
fn fadvise_06() {
    let mut path = crate::test_dir();
    path.push("fadvise_06.txt");

    let data = vec![0u8; 1024];
    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err =
        unsafe { libc::posix_fadvise(fd, 0, 1024, libc::POSIX_FADV_WILLNEED) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// cprange_01: Copy whole file to EOF
#[cfg(target_os = "linux")]
#[test]
fn cprange_01() {
    let mut path = crate::test_dir();
    path.push("cprange_01.txt");

    let data = vec![0u8; 1024];
    crate::create_file(&mut path, &data);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR) };
    assert!(fd > 0);

    let mut src_offset: i64 = 0;
    let mut dst_offset: i64 = 1024;
    let len = unsafe {
        libc::copy_file_range(fd, &mut src_offset, fd, &mut dst_offset, 1024, 0)
    };
    assert_eq!(len, 1024);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// cprange_02: Copy middle of file to new file
#[cfg(target_os = "linux")]
#[test]
fn cprange_02() {
    let mut path_src = crate::test_dir();
    let mut path_dst = path_src.clone();

    path_src.push("cprange_02_src.txt");
    path_dst.push("cprange_02_dst.txt");

    let data = vec![0u8; 4096];
    crate::create_file(&mut path_src, &data);
    crate::create_file(&mut path_dst, &[]);

    let err =
        unsafe { libc::chmod(path_dst.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let src_fd = unsafe { libc::open(path_src.c_str(), libc::O_RDONLY) };
    assert!(src_fd > 0);

    let dst_fd = unsafe { libc::open(path_dst.c_str(), libc::O_WRONLY) };
    assert!(dst_fd > 0);

    let mut src_offset: i64 = 1024;
    let mut dst_offset: i64 = 0;
    let len = unsafe {
        libc::copy_file_range(
            src_fd,
            &mut src_offset,
            dst_fd,
            &mut dst_offset,
            2048,
            0,
        )
    };
    assert_eq!(len, 2048);

    let err = unsafe { libc::close(src_fd) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(dst_fd) };
    assert_eq!(err, 0);

    let st = crate::stat(&mut path_dst);
    assert_eq!(st.st_size, 2048);
}
