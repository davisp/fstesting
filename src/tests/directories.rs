#[test]
fn mkdir() {
    let mut dir = crate::test_dir();

    for mode in crate::enums::FILE_PERMISSIONS {
        dir.push(format!("test_{mode}"));

        let err = unsafe { libc::mkdir(dir.c_str(), *mode) };
        assert_eq!(err, 0);

        dir.pop();
    }
}

#[test]
fn opendir_closedir() {
    let mut dir = crate::test_dir();
    dir.push("some_dir");

    let err = unsafe { libc::mkdir(dir.c_str(), urwx()) };
    assert_eq!(err, 0);

    let dir = unsafe { libc::opendir(dir.c_str()) };
    assert_ne!(dir, std::ptr::null_mut());

    let err = unsafe { libc::closedir(dir) };
    assert_eq!(err, 0);
}

#[test]
fn readdir() {
    let mut dir = crate::test_dir();

    let mut names = Vec::new();
    names.push(".".to_owned());
    names.push("..".to_owned());

    for i in 0..10 {
        let dname = format!("{i}");
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

#[test]
fn rmdir() {
    let mut dir = crate::test_dir();
    let err = unsafe { libc::rmdir(dir.c_str()) };
    assert_eq!(err, 0);
}

#[test]
fn fsyncdir() {
    let mut dir = crate::test_dir();

    let fd = unsafe { libc::open(dir.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let err = unsafe { libc::fsync(fd) };
    assert_eq!(err, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

#[cfg(not(target_os = "linux"))]
#[test]
#[ignore = "Not supported"]
fn fdatasyncdir() {}

#[cfg(target_os = "linux")]
#[test]
fn fdatasyncdir() {
    let mut dir = crate::test_dir();

    let fd = unsafe { libc::open(dir.c_str(), libc::O_RDWR) };
    let err = unsafe { libc::fdatasync(fd) };
    assert_eq!(err, -1);
    assert_eq!(crate::errno(), libc::EISDIR);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

fn urwx() -> libc::mode_t {
    libc::S_IRUSR | libc::S_IWUSR | libc::S_IXUSR
}
