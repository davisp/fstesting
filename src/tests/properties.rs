#[test]
fn access() {
    let mut path = crate::test_dir();
    path.push("access.txt");
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

#[test]
fn chmod() {
    let mut path = crate::test_dir();
    path.push("chmod.txt");
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

#[test]
fn utime() {
    let mut path = crate::test_dir();
    path.push("utime.txt");
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

#[test]
fn stat() {
    let mut path = crate::test_dir();
    path.push("stat.txt");
    crate::create_file(&mut path, &[]);

    let stat = crate::stat(&mut path);
    assert!(stat.st_ino != 0);
    assert_eq!(stat.st_size, 0);
}

#[test]
fn link() {
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

#[test]
fn symlink_readlink() {
    let mut src = crate::test_dir();
    src.push("symlink_readlink_src.txt");
    crate::create_file(&mut src, &[]);

    let mut dst = src.clone();
    dst.pop();
    dst.push("symlink_readlink_dst.txt");

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
    assert!(path.ends_with("symlink_readlink_src.txt"));
}

#[test]
fn rename() {
    let mut src = crate::test_dir();
    src.push("rename_src.txt");
    crate::create_file(&mut src, &[]);

    let mut dst = src.clone();
    dst.pop();
    dst.push("rename_dst.txt");

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

#[cfg(target_os = "macos")]
#[test]
fn xattrs() {
    let mut path = crate::test_dir();
    path.push("xattrs.txt");
    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let names = crate::listxattr(&mut path);
    assert!(names.is_empty());

    let err = unsafe {
        libc::setxattr(
            path.c_str(),
            c"foo".as_ptr() as *const libc::c_char,
            "bar".as_ptr() as *const libc::c_void,
            3,
            0,
            0,
        )
    };
    assert_eq!(err, 0);

    let names = crate::listxattr(&mut path);
    assert_eq!(names, &["foo"]);

    let mut value = vec![0u8; 2048];
    let len = unsafe {
        libc::getxattr(
            path.c_str(),
            c"foo".as_ptr() as *const libc::c_char,
            value.as_mut_ptr() as *mut libc::c_void,
            value.len(),
            0,
            0,
        )
    };
    assert!(len >= 0);

    let value = value
        .into_iter()
        .take_while(|c| *c != 0)
        .collect::<Vec<_>>();
    let value = String::from_utf8_lossy(&value).to_string();
    assert_eq!(value, "bar");

    let err = unsafe {
        libc::removexattr(
            path.c_str(),
            c"foo".as_ptr() as *const libc::c_char,
            0,
        )
    };
    assert_eq!(err, 0);

    let names = crate::listxattr(&mut path);
    assert!(names.is_empty());
}
