use std::sync::mpsc;

use rand::seq::SliceRandom;

use super::DATA_SIZE;
use crate::file_size;

// PWRITE1: Write file linearly
#[test]
fn pwrite_01() {
    let mut path = crate::test_dir();
    path.push("pwrite_01.txt");

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

/// PWRITE2: Create empty file, open and write linearly
#[test]
fn pwrite_02() {
    let mut path = crate::test_dir();
    path.push("pwrite_02.txt");

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

// PWRITE3: Write file, rewrite first half
#[test]
fn pwrite_03() {
    let mut path = crate::test_dir();
    path.push("pwrite_05.txt");

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

/// PWRITE4: Write half, close, open, rewrite second half.
#[test]
fn pwrite_04() {
    let mut path = crate::test_dir();
    path.push("pwrite_04.txt");

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

/// PWRITE5: Write half, reopen, write second half
#[test]
fn pwrite_05() {
    let mut path = crate::test_dir();
    path.push("pwrite_05.txt");

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

/// PWRITE6: Interleaved writes
#[test]
fn pwrite_06() {
    let mut path = crate::test_dir();
    path.push("pwrite_06.txt");

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

/// PWRITE7: Write file backwards
#[test]
fn pwrite_07() {
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

/// PWRITE8: Scattered writes
#[test]
fn pwrite_08() {
    let mut path = crate::test_dir();
    path.push("pwrite_08.txt");

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

/// PWRITE9: Parallel writes to same fd
#[test]
fn pwrite_09() {
    let mut path = crate::test_dir();
    path.push("pwrite_09.txt");

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

/// PWRITE10: Parallel writes to separate fds
#[test]
fn pwrite_10() {
    let mut path = crate::test_dir();
    path.push("pwrite_10.txt");

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

/// PWRITE11: parallel scattered writes same fd
#[test]
fn pwrite_11() {
    let mut path = crate::test_dir();
    path.push("pwrite_11.txt");

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

/// PWRITE12: parallel scattered writes separate fds
#[test]
fn pwrite_12() {
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

/// PWRITE13: Parallel overlapping writes with same contents
#[test]
fn pwrite_13() {
    let mut path = crate::test_dir();
    path.push("pwrite_13.txt");

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

/// PWRITE14: Parallel overlapping writes with different contents
#[test]
fn pwrite_14() {
    let mut path = crate::test_dir();
    path.push("pwrite_14.txt");

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

/// PWRITE15: pwrite unaffected by O_APPEND
#[cfg(target_os = "macos")]
#[test]
fn pwrite_15() {
    let mut path = crate::test_dir();
    path.push("pwrite_15.txt");
    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR | libc::O_APPEND) };
    assert!(fd > 0);

    let bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::pwrite(fd, bytes.as_ptr() as *const libc::c_void, bytes.len(), 0)
    };
    assert_eq!(len, bytes.len() as isize);

    let len = unsafe {
        libc::pwrite(fd, bytes.as_ptr() as *const libc::c_void, bytes.len(), 0)
    };
    assert_eq!(len, bytes.len() as isize);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let st = crate::stat(&mut path);
    assert_eq!(st.st_size, 1024);
}

/// PWRITE16: pwrite *is* affected by O_APPEND
#[cfg(target_os = "linux")]
#[test]
fn pwrite_16() {
    let mut path = crate::test_dir();
    path.push("pwrite_16.txt");
    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe { libc::open(path.c_str(), libc::O_RDWR | libc::O_APPEND) };
    assert!(fd > 0);

    let bytes = vec![0u8; 1024];
    let len = unsafe {
        libc::pwrite(fd, bytes.as_ptr() as *const libc::c_void, bytes.len(), 0)
    };
    assert_eq!(len, bytes.len() as isize);

    let len = unsafe {
        libc::pwrite(fd, bytes.as_ptr() as *const libc::c_void, bytes.len(), 0)
    };
    assert_eq!(len, bytes.len() as isize);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);

    let st = crate::stat(&mut path);
    assert_eq!(st.st_size, 2048);
}
