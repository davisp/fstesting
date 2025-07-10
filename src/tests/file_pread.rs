use rand::prelude::*;

use super::DATA_SIZE;

/// pread_01: Read a file 13 bytes at a time and check EOF behavior
#[test]
fn pread_01() {
    let mut path = crate::test_dir();
    path.push("pread_01.txt");

    let mut data = vec![0u8; DATA_SIZE];
    for (i, val) in data.iter_mut().enumerate() {
        let char: u8 = 97 + (i % 26) as u8;
        *val = char;
    }

    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    for idx in (0..DATA_SIZE).step_by(26) {
        let mut bytes = vec![0u8; 13];
        let len = unsafe {
            libc::pread(
                fd,
                bytes.as_mut_ptr() as *mut libc::c_void,
                bytes.len(),
                idx as i64,
            )
        };
        let should_read = std::cmp::min(13, DATA_SIZE - idx);
        assert_eq!(len, should_read as isize);
        assert_eq!(
            &bytes[..should_read],
            "abcdefghijklm"[..should_read].as_bytes()
        );

        if (idx + 13) > DATA_SIZE {
            break;
        }

        let len = unsafe {
            libc::pread(
                fd,
                bytes.as_mut_ptr() as *mut libc::c_void,
                bytes.len(),
                (idx + 13) as i64,
            )
        };
        let should_read = std::cmp::min(13, DATA_SIZE - idx);
        assert_eq!(len, should_read as isize);
        assert_eq!(
            &bytes[..should_read],
            "nopqrstuvwxyz"[..should_read].as_bytes()
        );
    }

    let pos = unsafe { libc::lseek(fd, 0, libc::SEEK_SET) };
    assert_eq!(pos, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// pread_02: Read 13 bytes at a time in two separate threads.
#[test]
fn pread_02() {
    let mut path = crate::test_dir();
    path.push("pread_02.txt");

    let mut data = vec![0u8; DATA_SIZE];
    for (i, val) in data.iter_mut().enumerate() {
        let char: u8 = 97 + (i % 26) as u8;
        *val = char;
    }

    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let t1 = std::thread::spawn(move || {
        let mut bytes = vec![0u8; 13];
        for idx in (0..DATA_SIZE).step_by(26) {
            let len = unsafe {
                libc::pread(
                    fd,
                    bytes.as_mut_ptr() as *mut libc::c_void,
                    bytes.len(),
                    idx as i64,
                )
            };
            let should_read = std::cmp::min(13, DATA_SIZE - idx);
            assert_eq!(len, should_read as isize);
            assert_eq!(
                &bytes[..should_read],
                "abcdefghijklm"[..should_read].as_bytes()
            );
        }
    });

    let t2 = std::thread::spawn(move || {
        let mut bytes = vec![0u8; 13];
        for idx in (13..DATA_SIZE).step_by(26) {
            let len = unsafe {
                libc::pread(
                    fd,
                    bytes.as_mut_ptr() as *mut libc::c_void,
                    bytes.len(),
                    idx as i64,
                )
            };
            let should_read = std::cmp::min(13, DATA_SIZE - idx);
            assert_eq!(len, should_read as isize);
            assert_eq!(
                &bytes[..should_read],
                "nopqrstuvwxyz"[..should_read].as_bytes()
            );
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// pread_03: Two threads read 13 bytes at a time using the same fd
#[test]
fn pread_03() {
    let mut path = crate::test_dir();
    path.push("pread_03.txt");

    let mut data = vec![0u8; DATA_SIZE];
    for (i, val) in data.iter_mut().enumerate() {
        let char: u8 = 97 + (i % 26) as u8;
        *val = char;
    }

    crate::create_file(&mut path, &data);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let t1 = std::thread::spawn(move || {
        let mut rng = rand::rng();
        let mut bytes = vec![0u8; 13];
        for _ in 0..2048 {
            let pos = rng.random_range(0..=(DATA_SIZE / 26));
            let len = unsafe {
                libc::pread(
                    fd,
                    bytes.as_mut_ptr() as *mut libc::c_void,
                    bytes.len(),
                    (pos * 26) as i64,
                )
            };
            let should_read = std::cmp::min(13, DATA_SIZE - (pos * 26));
            assert_eq!(len, should_read as isize);
            assert_eq!(
                &bytes[..should_read],
                "abcdefghijklm"[..should_read].as_bytes()
            );
        }
    });

    let t2 = std::thread::spawn(move || {
        let mut rng = rand::rng();
        let mut bytes = vec![0u8; 13];
        for _ in 0..2048 {
            let pos = rng.random_range(0..(DATA_SIZE / 26));
            let len = unsafe {
                libc::pread(
                    fd,
                    bytes.as_mut_ptr() as *mut libc::c_void,
                    bytes.len(),
                    ((pos * 26) + 13) as i64,
                )
            };
            let should_read = std::cmp::min(13, DATA_SIZE - ((pos * 26) + 13));
            assert_eq!(len, should_read as isize);
            assert_eq!(
                &bytes[..should_read],
                "nopqrstuvwxyz"[..should_read].as_bytes()
            );
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// pread_04: Two threads read 13 bytes at a time, using different fds
#[test]
fn pread_04() {
    let mut path = crate::test_dir();
    path.push("pread_04.txt");

    let mut data = vec![0u8; DATA_SIZE];
    for (i, val) in data.iter_mut().enumerate() {
        let char: u8 = 97 + (i % 26) as u8;
        *val = char;
    }

    crate::create_file(&mut path, &data);

    let mut p1 = path.clone();
    let t1 = std::thread::spawn(move || {
        let fd = unsafe { libc::open(p1.c_str(), libc::O_RDONLY) };
        assert!(fd > 0);

        let mut rng = rand::rng();
        let mut bytes = vec![0u8; 13];
        for _ in 0..2048 {
            let pos = rng.random_range(0..=(DATA_SIZE / 26));
            let len = unsafe {
                libc::pread(
                    fd,
                    bytes.as_mut_ptr() as *mut libc::c_void,
                    bytes.len(),
                    (pos * 26) as i64,
                )
            };
            let should_read = std::cmp::min(13, DATA_SIZE - (pos * 26));
            assert_eq!(len, should_read as isize);
            assert_eq!(
                &bytes[..should_read],
                "abcdefghijklm"[..should_read].as_bytes()
            );
        }

        let err = unsafe { libc::close(fd) };
        assert_eq!(err, 0);
    });

    let mut p2 = path.clone();
    let t2 = std::thread::spawn(move || {
        let fd = unsafe { libc::open(p2.c_str(), libc::O_RDONLY) };
        assert!(fd > 0);

        let mut rng = rand::rng();
        let mut bytes = vec![0u8; 13];
        for _ in 0..2048 {
            let pos = rng.random_range(0..(DATA_SIZE / 26));
            let len = unsafe {
                libc::pread(
                    fd,
                    bytes.as_mut_ptr() as *mut libc::c_void,
                    bytes.len(),
                    ((pos * 26) + 13) as i64,
                )
            };
            let should_read = std::cmp::min(13, DATA_SIZE - ((pos * 26) + 13));
            assert_eq!(len, should_read as isize);
            assert_eq!(
                &bytes[..should_read],
                "nopqrstuvwxyz"[..should_read].as_bytes()
            );
        }

        let err = unsafe { libc::close(fd) };
        assert_eq!(err, 0);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

/// pread_05: Read beyond EOF
#[test]
fn pread_05() {
    let mut path = crate::test_dir();
    path.push("pread_05.txt");
    crate::create_file(&mut path, "Hello, World!".as_bytes());

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd >= 0);

    let mut bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::pread(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len(), 0)
    };
    assert_eq!(len, "Hello, World!".len() as isize);
    assert_eq!(&bytes[..13], "Hello, World!".as_bytes());
    assert!(bytes[13..].iter().all(|c| *c == 0));

    let mut bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::pread(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len(), 7)
    };
    assert_eq!(len, 6);
    assert_eq!(&bytes[..6], "World!".as_bytes());
    assert!(bytes[6..].iter().all(|c| *c == 0));

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// pread_06: Error reading directory
#[test]
fn pread_06() {
    let mut path = crate::test_dir();

    let fd =
        unsafe { libc::open(path.c_str(), libc::O_RDONLY | libc::O_DIRECTORY) };
    assert!(fd >= 0);

    let mut bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::pread(fd, bytes.as_mut_ptr() as *mut libc::c_void, bytes.len(), 0)
    };
    assert_eq!(len, -1);
    assert_eq!(crate::errno(), libc::EISDIR);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// pread_08: EINVAL for pread with a negative offset
#[test]
fn pread_08() {
    let mut path = crate::test_dir();

    let fd =
        unsafe { libc::open(path.c_str(), libc::O_RDONLY | libc::O_DIRECTORY) };
    assert!(fd >= 0);

    let mut bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::pread(
            fd,
            bytes.as_mut_ptr() as *mut libc::c_void,
            bytes.len(),
            -1,
        )
    };
    assert_eq!(len, -1);
    assert_eq!(crate::errno(), libc::EINVAL);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}
