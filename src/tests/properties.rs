/// mdata_01: access
#[test]
fn mdata_01() {
    let mut path = crate::test_dir();
    path.push("mdata_01.txt");
    crate::create_file(&mut path, &[]);

    for mode in crate::enums::ACCESS_MODES {
        let err = unsafe { libc::access(path.c_str(), *mode) };
        if err == 0 {
            continue;
        }

        assert_eq!(err, -1);
        assert_eq!(crate::errno(), libc::EACCES);
    }
}

/// mdata_02: chmod
#[test]
fn mdata_02() {
    let mut path = crate::test_dir();
    path.push("mdata_02.txt");
    crate::create_file(&mut path, &[]);

    let stat = crate::stat(&mut path);
    assert_eq!(stat.st_mode & 0x0FFF, libc::S_IRUSR);

    for mode in crate::enums::FILE_PERMISSIONS {
        let err = unsafe { libc::chmod(path.c_str(), *mode) };
        assert_eq!(err, 0);

        let stat = crate::stat(&mut path);
        assert_eq!(stat.st_mode & 0x0FFF, *mode);
    }
}

/// mdata_03: utime
#[test]
fn mdata_03() {
    let mut path = crate::test_dir();
    path.push("mdata_03.txt");
    crate::create_file(&mut path, &[]);

    let err = unsafe {
        let mut times: [libc::timeval; 2] = std::mem::zeroed();

        times[0].tv_sec = 1;
        times[0].tv_usec = 1;
        times[1].tv_sec = 1;
        times[1].tv_usec = 3;

        libc::utimes(path.c_str(), &times[0])
    };

    let stat = crate::stat(&mut path);

    assert_eq!(stat.st_atime_nsec, 1000);
    assert_eq!(stat.st_mtime_nsec, 3000);

    assert_eq!(err, 0);
}

/// mdata_04: stat
#[test]
fn mdata_04() {
    let mut path = crate::test_dir();
    path.push("mdata_04.txt");
    crate::create_file(&mut path, &[]);

    let stat = crate::stat(&mut path);
    assert!(stat.st_ino != 0);
    assert_eq!(stat.st_size, 0);
}

/// mdata_05: link
#[test]
fn mdata_05() {
    let mut src = crate::test_dir();
    src.push("link_src.txt");
    crate::create_file(&mut src, &[]);

    let mut dst = src.clone();
    dst.pop();
    dst.push("link_dst.txt");

    let err = unsafe { libc::link(src.c_str(), dst.c_str()) };
    assert_eq!(err, 0);

    let err = unsafe { libc::access(dst.c_str(), libc::F_OK) };
    assert_eq!(err, 0);

    let src_stat = crate::stat(&mut src);
    let dst_stat = crate::stat(&mut dst);

    assert_eq!(src_stat.st_ino, dst_stat.st_ino);
    assert_eq!(src_stat.st_nlink, 2);
    assert_eq!(dst_stat.st_nlink, 2);
}

/// mdata_06: symlink/readlink
#[test]
fn mdata_06() {
    let mut src = crate::test_dir();
    src.push("mdata_06_src.txt");
    crate::create_file(&mut src, &[]);

    let mut dst = src.clone();
    dst.pop();
    dst.push("mdata_06_dst.txt");

    let err = unsafe { libc::symlink(src.c_str(), dst.c_str()) };
    assert_eq!(err, 0);

    let src_stat = crate::stat(&mut src);
    let dst_stat = crate::lstat(&mut dst);

    assert_ne!(src_stat.st_ino, dst_stat.st_ino);

    let mut data = vec![0u8; 1024];
    let len = unsafe {
        libc::readlink(
            dst.c_str(),
            data.as_mut_ptr() as *mut libc::c_char,
            1024,
        )
    };
    assert!(len > 0);

    let path = String::from_utf8_lossy(&data[..(len as usize)]).to_string();
    assert!(path.ends_with("mdata_06_src.txt"));
}

/// mdata_07: rename
#[test]
fn mdata_07() {
    let mut src = crate::test_dir();
    src.push("mdata_07_src.txt");
    crate::create_file(&mut src, &[]);

    let mut dst = src.clone();
    dst.pop();
    dst.push("mdata_07_dst.txt");

    let err = unsafe { libc::access(src.c_str(), libc::F_OK) };
    assert_eq!(err, 0);

    let err = unsafe { libc::access(dst.c_str(), libc::F_OK) };
    assert_eq!(err, -1);
    assert_eq!(crate::errno(), libc::ENOENT);

    let err = unsafe { libc::rename(src.c_str(), dst.c_str()) };
    assert_eq!(err, 0);

    let err = unsafe { libc::access(src.c_str(), libc::F_OK) };
    assert_eq!(err, -1);
    assert_eq!(crate::errno(), libc::ENOENT);

    let err = unsafe { libc::access(dst.c_str(), libc::F_OK) };
    assert_eq!(err, 0);
}
