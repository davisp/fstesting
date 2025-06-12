use super::DATA_SIZE;

/// READ1: Read a file 13 bytes at a time and check correct EOF behavior.
#[test]
fn read_01() {
    let mut path = crate::test_dir();
    path.push("read_01.txt");

    let mut data = vec![0u8; DATA_SIZE];
    for (i, val) in data.iter_mut().enumerate() {
        let char: u8 = 97 + (i % 26) as u8;
        *val = char;
    }

    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    for _ in 0..(DATA_SIZE / 26) {
        let mut bytes = vec![0u8; 13];
        let len = unsafe {
            libc::read(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
        };
        assert_eq!(len, 13);
        assert_eq!(bytes, "abcdefghijklm".as_bytes());

        let len = unsafe {
            libc::read(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
        };
        assert_eq!(len, 13);
        assert_eq!(bytes, "nopqrstuvwxyz".as_bytes());
    }

    assert_eq!(DATA_SIZE % 26, 18);
    let mut bytes = vec![0u8; 18];
    let len = unsafe {
        libc::read(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
    };
    assert_eq!(len, 18);
    assert_eq!(bytes, "abcdefghijklmnopqr".as_bytes());

    let len = unsafe {
        libc::read(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
    };
    assert_eq!(len, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// READ2: Read beyond EOF
#[test]
fn read_02() {
    let mut path = crate::test_dir();
    path.push("read_02.txt");
    crate::create_file(&mut path, "Hello, World!".as_bytes());

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd >= 0);

    let mut bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::read(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
    };
    assert_eq!(len, "Hello, World!".len() as isize);
    assert_eq!(&bytes[..13], "Hello, World!".as_bytes());
    assert!(bytes[13..].iter().all(|c| *c == 0));

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// READ3: Check for error when attempting to read a directory fd
#[test]
fn read_03() {
    let mut path = crate::test_dir();

    let fd =
        unsafe { libc::open(path.c_str(), libc::O_RDONLY | libc::O_DIRECTORY) };
    assert!(fd >= 0);

    let mut bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::read(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len())
    };
    assert_eq!(len, -1);
    assert_eq!(crate::errno(), libc::EISDIR);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}
