use rand::prelude::*;

use super::DATA_SIZE;
use crate::file_size;

// W1: Write file linearly
#[test]
fn write() {
    let mut path = crate::test_dir();
    path.push("write.txt");

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

/// W2: Create empty file, open and write linearly
#[test]
fn write_to_empty_file() {
    let mut path = crate::test_dir();
    path.push("write_to_empty_file.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_RDONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

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

// W3: Write file, rewrite first half
#[test]
fn write_and_rewrite_first_half() {
    let mut path = crate::test_dir();
    path.push("write_and_rewrite_first_half.txt");

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

/// W4: Write half, close, open, rewrite second half.
#[test]
fn write_reopen_rewrite_second_half() {
    let mut path = crate::test_dir();
    path.push("write_repopen_rewrite_second_half.txt");

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

/// W5: Write half, reopen, write second half
#[test]
fn write_half_reopen_write_second_half() {
    let mut path = crate::test_dir();
    path.push("write_half_reopen_write_second_half.txt");

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

/// W6: Interleaved writes
#[test]
fn write_interleaved() {
    let mut path = crate::test_dir();
    path.push("write_interleaved.txt");

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

/// W7: Write file backwards
#[test]
fn write_backwards() {
    let mut path = crate::test_dir();
    path.push("write_backwards.txt");

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

/// W8: write scattered positions
#[test]
fn write_scattered() {
    let mut path = crate::test_dir();
    path.push("write_scattered.txt");

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

/// W9: Parallel writes to same fd
#[test]
fn write_parallel_same_fd() {
    let mut path = crate::test_dir();
    path.push("write_parallel_same_fd.txt");

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

/// W10: Parallel writes to separate fds
#[test]
fn write_parallel_separate_fds() {
    let mut path = crate::test_dir();
    path.push("write_parallel_separate_fds.txt");

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
