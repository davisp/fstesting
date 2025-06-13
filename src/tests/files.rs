use rand::prelude::*;

const DATA_SIZE: usize = 1024 * 1024 * 15;

#[test]
fn unlink() {
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

/// R1: Read a file 13 bytes at a time and check correct EOF behavior.
#[test]
fn read() {
    let mut path = crate::test_dir();
    path.push("read.txt");

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

/// R2: Read beyond EOF
#[test]
fn read_partial() {
    let mut path = crate::test_dir();
    path.push("read_partial.txt");
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

/// R3: Check for error when attempting to read a directory fd
#[test]
fn read_directory() {
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

// PR1: Read a file 13 bytes at a time and check EOF behavior
#[test]
fn pread() {
    let mut path = crate::test_dir();
    path.push("read.txt");

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
        assert_eq!(len, 13);
        assert_eq!(bytes, "abcdefghijklm".as_bytes());

        let tail = std::cmp::min(13, DATA_SIZE - idx);
        let len = unsafe {
            libc::pread(
                fd,
                bytes.as_mut_ptr() as *mut libc::c_void,
                bytes.len(),
                tail as i64,
            )
        };
        assert_eq!(len, tail as isize);
        assert_eq!(bytes, "nopqrstuvwxyz"[..tail].as_bytes());
    }

    let pos = unsafe { libc::lseek(fd, 0, libc::SEEK_SET) };
    assert_eq!(pos, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

// PR2: Read 13 bytes at a time in two separate threads.
#[test]
fn pread_parallel_same_fd() {
    let mut path = crate::test_dir();
    path.push("read.txt");

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
            assert_eq!(len, 13);
            assert_eq!(bytes, "abcdefghijklm".as_bytes());
        }
    });

    let t2 = std::thread::spawn(move || {
        let mut bytes = vec![0u8; 13];
        for idx in (0..DATA_SIZE).step_by(26) {
            let tail = std::cmp::min(13, DATA_SIZE - idx);
            let len = unsafe {
                libc::pread(
                    fd,
                    bytes.as_mut_ptr() as *mut libc::c_void,
                    bytes.len(),
                    tail as i64,
                )
            };
            assert_eq!(len, tail as isize);
            assert_eq!(bytes, "nopqrstuvwxyz"[..tail].as_bytes());
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// PR3: Two threads read 13 bytes at a time using the same fd
#[test]
fn pread_parallel_scatter_same_fd() {
    let mut path = crate::test_dir();
    path.push("read.txt");

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
            assert_eq!(len, 13);
            assert_eq!(bytes, "abcdefghijklm".as_bytes());
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
            assert_eq!(len, 13);
            assert_eq!(bytes, "nopqrstuvwxyz".as_bytes());
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// PR4: Two threads read 13 bytes at a time, using different fds
#[test]
fn pread_parallel_scatter_separate_fd() {
    let mut path = crate::test_dir();
    path.push("read.txt");

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
            assert_eq!(len, 13);
            assert_eq!(bytes, "abcdefghijklm".as_bytes());
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
            assert_eq!(len, 13);
            assert_eq!(bytes, "nopqrstuvwxyz".as_bytes());
        }

        let err = unsafe { libc::close(fd) };
        assert_eq!(err, 0);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

/// PR5: Read beyond EOF
#[test]
fn pread_partial() {
    let mut path = crate::test_dir();
    path.push("read_partial.txt");
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

/// PR6: Error reading directory
#[test]
fn pread_directory() {
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

/// PR8: EINVAL for pread with a negative offset
#[test]
fn pread_einval() {
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

// write file linearly
// write file, rewrite first half
// write half, close, open, write second half.
// write file linearly, close, open, rewrite first half
// write interleaved, write every other 13 bytes, seek 13, then fill in missing bytes
// write backwards, seek to end-13, write, seek -13
// parallel write, two threads attempt to write same file, separate fds

// All the write tests, but with pwrite
// scatter writes, fill in entire file with random writes (rng shuffle DSIZE/26)
// parallel pwrite, same fd, interleaved writes,
// parallel pwrite, separate fds
// parallel scatter write, same as serial, but give half positions to each thread
// parallel scatter write, same as serial, but give half positions to each thread, separate fds
// parallel overlap writes, two threads sync up every N writes, both writing same contents
// parallel overlap writes, two threads sync up every N writes, write different contents

// - write()
// - pwrite()
// - truncate()
// - copy_file_range()
// - lseek()
// - fadvise
// - fallocate()
// - fcntl
// - flock()
// - flush()
// - fsync()
// - fdatasync()
