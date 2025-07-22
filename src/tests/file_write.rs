use rand::prelude::*;

use super::DATA_SIZE;
use crate::file_size;

/// write_01: Write file linearly
#[test]
fn write_01() {
    let mut path = crate::test_dir();
    path.push("write_01.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";

    for _ in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, DATA_SIZE as i64);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    assert_eq!(file_size(&mut path), DATA_SIZE);
}

/// write_02: Create empty file, open and write linearly
#[test]
fn write_02() {
    let mut path = crate::test_dir();
    path.push("write_02.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let len = unsafe {
        libc::write(
            fd,
            "foo".as_bytes().as_ptr() as *const libc::c_void,
            3
        )
    };
    assert_eq!(len, 3);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_WRONLY) };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    for _ in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, DATA_SIZE as i64);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// write_03: Write file, rewrite first half
#[test]
fn write_03() {
    let mut path = crate::test_dir();
    path.push("write_03.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    for _ in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, DATA_SIZE as i64);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_SET) };
    assert_eq!(offset, 0);

    for _ in 0..((DATA_SIZE / 26) / 2) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// write_04: Write half, close, open, rewrite second half.
#[test]
fn write_04() {
    let mut path = crate::test_dir();
    path.push("write_04.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    for _ in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, DATA_SIZE as i64);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_WRONLY) };
    assert!(fd > 0);

    for _ in ((DATA_SIZE / 26) / 2)..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// write_05: Write half, reopen, write second half
#[test]
fn write_05() {
    let mut path = crate::test_dir();
    path.push("write_05.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    for _ in 0..((DATA_SIZE / 26) / 2) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let written = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_WRONLY) };
    assert!(fd > 0);

    let offset = unsafe { libc::lseek(fd, written, libc::SEEK_SET) };
    assert_eq!(offset, written);

    for _ in ((DATA_SIZE / 26) / 2)..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, DATA_SIZE as i64);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// write_06: Interleaved writes
#[test]
fn write_06() {
    let mut path = crate::test_dir();
    path.push("write_06.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklm";
    for idx in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);

        let offset = unsafe { libc::lseek(fd, 13, libc::SEEK_CUR) };
        assert_eq!(offset, ((idx + 1) * 26) as i64);
    }

    let offset = unsafe { libc::lseek(fd, 13, libc::SEEK_SET) };
    assert_eq!(offset, 13);

    let bytes = "nopqrstuvwxyz";
    for idx in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);

        let offset = unsafe { libc::lseek(fd, 13, libc::SEEK_CUR) };
        assert_eq!(offset, ((idx + 1) * 26 + 13) as i64);
    }

    // Undo the last lseek of the previous loop
    let offset = unsafe {
        libc::lseek(fd, (DATA_SIZE / 26 * 26) as i64, libc::SEEK_SET)
    };
    assert_eq!(offset, (DATA_SIZE / 26 * 26) as i64);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, DATA_SIZE as i64);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// write_07: Write file backwards
#[test]
fn write_07() {
    let mut path = crate::test_dir();
    path.push("write_07.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let offset = unsafe {
        libc::lseek(fd, (DATA_SIZE / 26 * 26) as i64, libc::SEEK_SET)
    };
    assert_eq!(offset, (DATA_SIZE / 26 * 26) as i64);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    for idx in (0..(DATA_SIZE / 26)).rev() {
        let offset =
            unsafe { libc::lseek(fd, (idx * 26) as i64, libc::SEEK_SET) };
        assert_eq!(offset, (idx * 26) as i64);

        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 26);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// write_08: write scattered positions
#[test]
fn write_08() {
    let mut path = crate::test_dir();
    path.push("write_08.txt");

    let mut rng = rand::rng();
    let mut positions = (0..(DATA_SIZE / 26)).collect::<Vec<_>>();
    positions.shuffle(&mut rng);

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    for idx in positions {
        let offset =
            unsafe { libc::lseek(fd, (idx * 26) as i64, libc::SEEK_SET) };
        assert_eq!(offset, (idx * 26) as i64);

        let len = unsafe {
            libc::write(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let offset = unsafe {
        libc::lseek(fd, (DATA_SIZE / 26 * 26) as i64, libc::SEEK_SET)
    };
    assert_eq!(offset, (DATA_SIZE / 26 * 26) as i64);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, DATA_SIZE as i64);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// write_09: Parallel writes to same fd
#[test]
fn write_09() {
    let mut path = crate::test_dir();
    path.push("write_09.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let t1 = std::thread::spawn(move || {
        let bytes = "abcdefghijklm";
        for _ in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::write(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                )
            };
            assert_eq!(len, bytes.len() as isize);
        }
    });

    let t2 = std::thread::spawn(move || {
        let bytes = "nopqrstuvwxyz";
        for _ in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::write(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                )
            };
            assert_eq!(len, bytes.len() as isize);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, DATA_SIZE as i64);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// write_10: Parallel writes to separate fds
#[test]
fn write_10() {
    let mut path = crate::test_dir();
    path.push("write_10.txt");

    let mut p1 = path.clone();
    let t1 = std::thread::spawn(move || {
        let fd = unsafe {
            crate::wrappers::open3(
                p1.c_str(),
                libc::O_WRONLY | libc::O_CREAT,
                libc::S_IRUSR | libc::S_IWUSR,
            )
        };
        assert!(fd > 0);

        let bytes = "abcdefghijklm";
        for _ in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::write(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                )
            };
            assert_eq!(len, bytes.len() as isize);
        }

        let err = unsafe { libc::close(fd) };
        assert_eq!(err, 0);
    });

    let mut p2 = path.clone();
    let t2 = std::thread::spawn(move || {
        let fd = unsafe {
            crate::wrappers::open3(
                p2.c_str(),
                libc::O_WRONLY | libc::O_CREAT,
                libc::S_IRUSR | libc::S_IWUSR,
            )
        };
        assert!(fd > 0);

        let bytes = "nopqrstuvwxyz";
        for _ in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::write(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                )
            };
            assert_eq!(len, bytes.len() as isize);
        }

        let err = unsafe { libc::close(fd) };
        assert_eq!(err, 0);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let offset = unsafe {
        libc::lseek(fd, (DATA_SIZE / 26 * 26) as i64, libc::SEEK_SET)
    };
    assert_eq!(offset, (DATA_SIZE / 26 * 26) as i64);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::write(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, DATA_SIZE as i64);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// write_11: Check append only with truncate and write
#[test]
fn write_11() {
    let mut path = crate::test_dir();
    path.push("write_11.txt");

    crate::create_file_rw(&mut path, "Hello, World!".as_bytes());

    let fd = unsafe {
        libc::open(path.c_str(), libc::O_RDWR | libc::O_APPEND | libc::O_TRUNC)
    };
    assert!(fd > 0);

    let bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::write(fd, bytes.as_ptr() as *const libc::c_void, bytes.len())
    };
    assert_eq!(len, bytes.len() as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_SET) };
    assert_eq!(offset, 0);

    let len = unsafe {
        libc::write(fd, bytes.as_ptr() as *const libc::c_void, bytes.len())
    };
    assert_eq!(len, bytes.len() as isize);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let st = crate::stat(&mut path);
    assert_eq!(st.st_size, 2048);
}

/// write_12: Observer writes through separate fd
#[test]
fn write_12() {
    let mut path = crate::test_dir();
    path.push("write_12.txt");

    crate::create_file_rw(&mut path, &[0u8]);

    let mut p1 = path.clone();
    let t1 = std::thread::spawn(move || {
        let fd = unsafe { libc::open(p1.c_str(), libc::O_RDWR) };
        assert!(fd > 0);

        loop {
            let mut bytes = vec![0u8];
            let len = unsafe {
                libc::read(
                    fd,
                    bytes.as_mut_ptr() as *mut libc::c_void,
                    bytes.len(),
                )
            };

            if len == 0 {
                continue;
            }

            assert_eq!(len, bytes.len() as isize);

            if bytes[0] % 2 == 0 {
                bytes[0] += 1;
                let len = unsafe {
                    libc::write(
                        fd,
                        bytes.as_ptr() as *const libc::c_void,
                        bytes.len(),
                    )
                };
                assert_eq!(len, 1);
            }

            if bytes[0] >= 254 {
                break;
            }
        }

        let err = unsafe { libc::close(fd) };
        assert_eq!(err, 0);
    });

    let mut p2 = path.clone();
    let t2 = std::thread::spawn(move || {
        let fd = unsafe { libc::open(p2.c_str(), libc::O_RDWR) };
        assert!(fd > 0);

        loop {
            let mut bytes = vec![0u8];
            let len = unsafe {
                libc::read(
                    fd,
                    bytes.as_mut_ptr() as *mut libc::c_void,
                    bytes.len(),
                )
            };

            if len == 0 {
                continue;
            }

            assert_eq!(len, bytes.len() as isize);

            if bytes[0] >= 254 {
                break;
            }

            if bytes[0] % 2 == 1 {
                bytes[0] += 1;
                let len = unsafe {
                    libc::write(
                        fd,
                        bytes.as_ptr() as *const libc::c_void,
                        bytes.len(),
                    )
                };
                assert_eq!(len, 1);
            }
        }

        let err = unsafe { libc::close(fd) };
        assert_eq!(err, 0);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let st = crate::stat(&mut path);
    assert_eq!(st.st_size, 256);
}
