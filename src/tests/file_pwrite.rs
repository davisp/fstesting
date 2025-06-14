use std::sync::mpsc;

use rand::seq::SliceRandom;

use super::DATA_SIZE;
use crate::file_size;

// PW1: Write file linearly
#[test]
fn pwrite() {
    let mut path = crate::test_dir();
    path.push("pwrite.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";

    for idx in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    assert_eq!(file_size(&mut path), DATA_SIZE);
}

/// PW2: Create empty file, open and write linearly
#[test]
fn pwrite_to_empty_file() {
    let mut path = crate::test_dir();
    path.push("pwrite_to_empty_file.txt");

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
    for idx in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

// PW3: Write file, rewrite first half
#[test]
fn pwrite_and_rewrite_first_half() {
    let mut path = crate::test_dir();
    path.push("pwrite_and_rewrite_first_half.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    for idx in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    for idx in 0..((DATA_SIZE / 26) / 2) {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW4: Write half, close, open, rewrite second half.
#[test]
fn pwrite_reopen_rewrite_second_half() {
    let mut path = crate::test_dir();
    path.push("pwrite_reopen_rewrite_second_half.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    for idx in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_WRONLY) };
    assert!(fd > 0);

    for idx in ((DATA_SIZE / 26) / 2)..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW5: Write half, reopen, write second half
#[test]
fn pwrite_half_reopen_write_second_half() {
    let mut path = crate::test_dir();
    path.push("pwrite_half_reopen_write_second_half.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    for idx in 0..((DATA_SIZE / 26) / 2) {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_WRONLY) };
    assert!(fd > 0);

    for idx in ((DATA_SIZE / 26) / 2)..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW6: Interleaved writes
#[test]
fn pwrite_interleaved() {
    let mut path = crate::test_dir();
    path.push("pwrite_interleaved.txt");

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
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let bytes = "nopqrstuvwxyz";
    for idx in 0..(DATA_SIZE / 26) {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26 + 13) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW7: Write file backwards
#[test]
fn pwrite_backwards() {
    let mut path = crate::test_dir();
    path.push("pwrite_backwards.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    for idx in (0..(DATA_SIZE / 26)).rev() {
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW8: Scattered writes
#[test]
fn pwrite_scattered() {
    let mut path = crate::test_dir();
    path.push("pwrite_scattered.txt");

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
        let len = unsafe {
            libc::pwrite(
                fd,
                bytes.as_bytes().as_ptr() as *const libc::c_void,
                bytes.len(),
                (idx * 26) as i64,
            )
        };
        assert_eq!(len, bytes.len() as isize);
    }

    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    assert_eq!(file_size(&mut path), DATA_SIZE);
}

/// PW9: Parallel writes to same fd
#[test]
fn pwrite_parallel_same_fd() {
    let mut path = crate::test_dir();
    path.push("pwrite_parallel_same_fd.txt");

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
        for idx in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26) as i64,
                )
            };
            assert_eq!(len, bytes.len() as isize);
        }
    });

    let t2 = std::thread::spawn(move || {
        let bytes = "nopqrstuvwxyz";
        for idx in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26 + 13) as i64,
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
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW10: Parallel writes to separate fds
#[test]
fn pwrite_parallel_separate_fds() {
    let mut path = crate::test_dir();
    path.push("pwrite_parallel_separate_fds.txt");

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
        for idx in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26) as i64,
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
        for idx in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26 + 13) as i64,
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

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW11: parallel scattered writes same fd
#[test]
fn pwrite_scattered_parallel_same_fd() {
    let mut path = crate::test_dir();
    path.push("pwrite_scattered_parallel_same_fd.txt");

    let mut rng = rand::rng();

    let mut t1_positions = (0..(DATA_SIZE / 26)).collect::<Vec<_>>();
    t1_positions.shuffle(&mut rng);

    let mut t2_positions = (0..(DATA_SIZE / 26)).collect::<Vec<_>>();
    t2_positions.shuffle(&mut rng);

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
        for idx in t1_positions {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26) as i64,
                )
            };
            assert_eq!(len, bytes.len() as isize);
        }
    });

    let t2 = std::thread::spawn(move || {
        let bytes = "nopqrstuvwxyz";
        for idx in t2_positions {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26 + 13) as i64,
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
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW12: parallel scattered writes separate fds
#[test]
fn pwrite_scattered_parallel_separate_fds() {
    let mut path = crate::test_dir();
    path.push("pwrite_scattered_parallel_separate_fds.txt");

    let mut rng = rand::rng();

    let mut t1_positions = (0..(DATA_SIZE / 26)).collect::<Vec<_>>();
    t1_positions.shuffle(&mut rng);

    let mut t2_positions = (0..(DATA_SIZE / 26)).collect::<Vec<_>>();
    t2_positions.shuffle(&mut rng);

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
        for idx in t1_positions {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26) as i64,
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
        for idx in t2_positions {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26 + 13) as i64,
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

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW13: Parallel overlapping writes with same contents
#[test]
fn pwrite_parallel_overlapping_same_contents() {
    let mut path = crate::test_dir();
    path.push("pwrite_parallel_overlapping_same_contents.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let (t1_tx, t2_rx) = mpsc::channel();
    let (t2_tx, t1_rx) = mpsc::channel();

    let t1 = std::thread::spawn(move || {
        let bytes = "abcdefghijklmnopqrstuvwxyz";
        for idx in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26) as i64,
                )
            };
            assert_eq!(len, bytes.len() as isize);

            if idx % 1000 == 0 {
                t1_tx.send(idx).unwrap();
                let t2_idx = t1_rx.recv().unwrap();
                assert_eq!(t2_idx, idx);
            }
        }
    });

    let t2 = std::thread::spawn(move || {
        let bytes = "abcdefghijklmnopqrstuvwxyz";
        for idx in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26) as i64,
                )
            };
            assert_eq!(len, bytes.len() as isize);

            if idx % 1000 == 0 {
                t2_tx.send(idx).unwrap();
                let t1_idx = t2_rx.recv().unwrap();
                assert_eq!(t1_idx, idx);
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}

/// PW14: Parallel overlapping writes with different contents
#[test]
fn pwrite_parallel_overlapping_different_contents() {
    let mut path = crate::test_dir();
    path.push("pwrite_parallel_overlapping_different_contents.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let (t1_tx, t2_rx) = mpsc::channel();
    let (t2_tx, t1_rx) = mpsc::channel();

    let t1 = std::thread::spawn(move || {
        let bytes = "abcdefghijklmnopqrstuvwxyz";
        for idx in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26) as i64,
                )
            };
            assert_eq!(len, bytes.len() as isize);

            if idx % 1000 == 0 {
                t1_tx.send(idx).unwrap();
                let t2_idx = t1_rx.recv().unwrap();
                assert_eq!(t2_idx, idx);
            }
        }
    });

    let t2 = std::thread::spawn(move || {
        let bytes = "ZYXWVUTSRQPONMLKJIHGFEDCBA";
        for idx in 0..(DATA_SIZE / 26) {
            let len = unsafe {
                libc::pwrite(
                    fd,
                    bytes.as_bytes().as_ptr() as *const libc::c_void,
                    bytes.len(),
                    (idx * 26) as i64,
                )
            };
            assert_eq!(len, bytes.len() as isize);

            if idx % 1000 == 0 {
                t2_tx.send(idx).unwrap();
                let t1_idx = t2_rx.recv().unwrap();
                assert_eq!(t1_idx, idx);
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let bytes = "abcdefghijklmnopqrstuvwxyz";
    let tail = DATA_SIZE % 26;
    let len = unsafe {
        libc::pwrite(
            fd,
            bytes.as_bytes()[..tail].as_ptr() as *const libc::c_void,
            tail,
            (DATA_SIZE / 26 * 26) as i64,
        )
    };
    assert_eq!(len, tail as isize);

    let offset = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    assert_eq!(offset, 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let size = file_size(&mut path);
    assert_eq!(size, DATA_SIZE);
}
