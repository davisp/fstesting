/// dirs_01: mkdir
#[test]
fn dirs_01() {
    let mut dir = crate::test_dir();

    for mode in crate::enums::FILE_PERMISSIONS {
        dir.push(format!("dirs_01_{mode}"));

        let err = unsafe { libc::mkdir(dir.c_str(), *mode) };
        assert_eq!(err, 0);

        dir.pop();
    }
}

/// dirs_02: opendir/closedir
#[test]
fn dirs_02() {
    let mut dir = crate::test_dir();
    dir.push("dirs_02");

    let err = unsafe { libc::mkdir(dir.c_str(), urwx()) };
    assert_eq!(err, 0);

    let dir = unsafe { libc::opendir(dir.c_str()) };
    assert_ne!(dir, std::ptr::null_mut());

    let err = unsafe { libc::closedir(dir) };
    assert_eq!(err, 0);
}

/// dirs_03: readdir
#[test]
fn dirs_03() {
    let mut dir = crate::test_dir();

    let mut names = Vec::new();
    names.push(".".to_owned());
    names.push("..".to_owned());

    for i in 0..10 {
        let dname = format!("dirs_03_{i}");
        names.push(dname.clone());
        dir.push(dname);

        let err = unsafe { libc::mkdir(dir.c_str(), urwx()) };
        assert_eq!(err, 0);

        dir.pop();
    }

    let dir = unsafe { libc::opendir(dir.c_str()) };
    assert_ne!(dir, std::ptr::null_mut());

    let mut found = Vec::new();
    loop {
        let entry = unsafe { libc::readdir(dir) };
        if entry.is_null() {
            break;
        }

        let name = unsafe { (*entry).d_name };
        let name = name
            .iter()
            .take_while(|c| **c != 0)
            .map(|c| *c as u8)
            .collect::<Vec<_>>();
        let name = String::from_utf8_lossy(&name).to_string();
        found.push(name);
    }

    let err = unsafe { libc::closedir(dir) };
    assert_eq!(err, 0);

    names.sort();
    found.sort();
    assert_eq!(found, names);
}

/// dirs_04: rmdir
#[test]
fn dirs_04() {
    let mut dir = crate::test_dir();
    let err = unsafe { libc::rmdir(dir.c_str()) };
    assert_eq!(err, 0);
}

/// dirs_05: fsync directory
#[test]
fn dirs_05() {
    let mut dir = crate::test_dir();

    let fd = unsafe { libc::open(dir.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err = unsafe { libc::fsync(fd) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// dirs_06: error creating file in non-executable directory
#[test]
fn dirs_06() {
    let mut dir = crate::test_dir();
    dir.push("dirs_06");

    let err =
        unsafe { libc::mkdir(dir.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let mut file = dir.clone();
    file.push("dirs_06.txt");

    let fd = unsafe {
        crate::open3(
            file.c_str(),
            libc::O_RDONLY | libc::O_CREAT,
            libc::S_IRUSR,
        )
    };
    assert_eq!(fd, -1);
    assert_eq!(crate::errno(), libc::EACCES);
}

/// dirs_07: fdatasync directory
#[cfg(target_os = "linux")]
#[test]
fn dirs_07() {
    let mut dir = crate::test_dir();

    let fd = unsafe { libc::open(dir.c_str(), libc::O_RDWR) };
    let err = unsafe { libc::fdatasync(fd) };
    assert_eq!(err, -1);
    assert_eq!(crate::errno(), libc::EBADF);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, -1);
    assert_eq!(crate::errno(), libc::EBADF);
}

fn urwx() -> libc::mode_t {
    libc::S_IRUSR | libc::S_IWUSR | libc::S_IXUSR
}
