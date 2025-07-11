#[macro_export]
macro_rules! open_ne {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (omode, oflags, success, errno) = $value;
            let mut path = $crate::test_dir();
            path.push("open_nonexistant.txt");

            let fd = unsafe { libc::open(path.c_str(), omode | oflags) };

            if success {
                assert!(fd >= 0);
                let err = unsafe { libc::close(fd) };
                assert_eq!(err, 0);
            } else {
                assert_eq!($crate::errno(), errno);
            }
        }
    )*
    }
}

#[macro_export]
macro_rules! open_creat {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (omode, oflags, success, errno) = $value;
            let mut path = $crate::test_dir();
            path.push("open_creat.txt");

            let fd = unsafe { libc::open(path.c_str(), omode | libc::O_CREAT | oflags) };

            if success {
                assert_eq!($crate::errno(), 0);
                assert!(fd >= 0);
                let err = unsafe { libc::close(fd) };
                assert_eq!(err, 0);
            } else {
                assert_eq!($crate::errno(), errno);
            }
        }
    )*
    }
}

#[macro_export]
macro_rules! open_exist_ro {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (omode, oflags, success, errno) = $value;
            let mut path = $crate::test_dir();
            path.push("open_existing_ro.txt");
            $crate::create_file(&mut path, &[]);

            let fd = unsafe { libc::open(path.c_str(), omode | oflags) };

            if success {
                assert_eq!(crate::errno(), 0);
                assert!(fd >= 0);
                let err = unsafe { libc::close(fd) };
                assert_eq!(err, 0);
            } else {
                assert_eq!($crate::errno(), errno);
            }
        }
    )*
    }
}

#[macro_export]
macro_rules! open_exist_rw {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (omode, oflags, success, errno) = $value;
            let mut path = $crate::test_dir();
            path.push("open_existing_rw.txt");
            $crate::create_file(&mut path, &[]);

            let err = unsafe {
                libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR)
            };
            assert_eq!(err, 0);

            let fd = unsafe { libc::open(path.c_str(), omode | oflags) };

            if success {
                assert!(fd >= 0);
                let err = unsafe { libc::close(fd) };
                assert_eq!(err, 0);
            } else {
                assert_eq!($crate::errno(), errno);
            }
        }
    )*
    }
}

pub mod common_open_ne {
    crate::open_ne! {
        open_ne_01: (libc::O_RDONLY, libc::O_NONBLOCK, false, libc::ENOENT),
        open_ne_02: (libc::O_RDONLY, libc::O_APPEND, false, libc::ENOENT),
        open_ne_03: (libc::O_RDONLY, libc::O_CREAT, true, 0),
        open_ne_04: (libc::O_RDONLY, libc::O_TRUNC, false, libc::ENOENT),
        open_ne_05: (libc::O_RDONLY, libc::O_EXCL, false, libc::ENOENT),
        open_ne_06: (libc::O_RDONLY, libc::O_NOFOLLOW, false, libc::ENOENT),
        open_ne_07: (libc::O_RDONLY, libc::O_CLOEXEC, false, libc::ENOENT),
        open_ne_08: (libc::O_WRONLY, libc::O_NONBLOCK, false, libc::ENOENT),
        open_ne_09: (libc::O_WRONLY, libc::O_APPEND, false, libc::ENOENT),
        open_ne_10: (libc::O_WRONLY, libc::O_CREAT, true, 0),
        open_ne_11: (libc::O_WRONLY, libc::O_TRUNC, false, libc::ENOENT),
        open_ne_12: (libc::O_WRONLY, libc::O_EXCL, false, libc::ENOENT),
        open_ne_13: (libc::O_WRONLY, libc::O_NOFOLLOW, false, libc::ENOENT),
        open_ne_14: (libc::O_WRONLY, libc::O_CLOEXEC, false, libc::ENOENT),
        open_ne_15: (libc::O_RDWR, libc::O_NONBLOCK, false, libc::ENOENT),
        open_ne_16: (libc::O_RDWR, libc::O_APPEND, false, libc::ENOENT),
        open_ne_17: (libc::O_RDWR, libc::O_CREAT, true, 0),
        open_ne_18: (libc::O_RDWR, libc::O_TRUNC, false, libc::ENOENT),
        open_ne_19: (libc::O_RDWR, libc::O_EXCL, false, libc::ENOENT),
        open_ne_20: (libc::O_RDWR, libc::O_NOFOLLOW, false, libc::ENOENT),
        open_ne_21: (libc::O_RDWR, libc::O_CLOEXEC, false, libc::ENOENT),
    }
}

#[cfg(target_os = "linux")]
pub mod linux_open_ne {
    crate::open_ne! {
        open_ne_22: (libc::O_RDONLY, libc::O_ASYNC, false, libc::ENOENT),
        open_ne_23: (libc::O_RDONLY, libc::O_DIRECT, false, libc::ENOENT),
        open_ne_24: (libc::O_RDONLY, libc::O_DSYNC, false, libc::ENOENT),
        open_ne_25: (libc::O_RDONLY, libc::O_LARGEFILE, false, libc::ENOENT),
        open_ne_26: (libc::O_RDONLY, libc::O_NOATIME, false, libc::ENOENT),
        open_ne_27: (libc::O_RDONLY, libc::O_NOCTTY, false, libc::ENOENT),
        open_ne_28: (libc::O_RDONLY, libc::O_NDELAY, false, libc::ENOENT),
        open_ne_29: (libc::O_RDONLY, libc::O_PATH, false, libc::ENOENT),
        open_ne_30: (libc::O_RDONLY, libc::O_SYNC, false, libc::ENOENT),
        open_ne_31: (libc::O_RDONLY, libc::O_TMPFILE, false, libc::EINVAL),
        open_ne_32: (libc::O_WRONLY, libc::O_ASYNC, false, libc::ENOENT),
        open_ne_33: (libc::O_WRONLY, libc::O_DIRECT, false, libc::ENOENT),
        open_ne_34: (libc::O_WRONLY, libc::O_DSYNC, false, libc::ENOENT),
        open_ne_35: (libc::O_WRONLY, libc::O_LARGEFILE, false, libc::ENOENT),
        open_ne_36: (libc::O_WRONLY, libc::O_NOATIME, false, libc::ENOENT),
        open_ne_37: (libc::O_WRONLY, libc::O_NOCTTY, false, libc::ENOENT),
        open_ne_38: (libc::O_WRONLY, libc::O_NDELAY, false, libc::ENOENT),
        open_ne_39: (libc::O_WRONLY, libc::O_PATH, false, libc::ENOENT),
        open_ne_40: (libc::O_WRONLY, libc::O_SYNC, false, libc::ENOENT),
        open_ne_41: (libc::O_WRONLY, libc::O_TMPFILE, false, libc::ENOENT),
        open_ne_42: (libc::O_RDWR, libc::O_ASYNC, false, libc::ENOENT),
        open_ne_43: (libc::O_RDWR, libc::O_DIRECT, false, libc::ENOENT),
        open_ne_44: (libc::O_RDWR, libc::O_DSYNC, false, libc::ENOENT),
        open_ne_45: (libc::O_RDWR, libc::O_LARGEFILE, false, libc::ENOENT),
        open_ne_46: (libc::O_RDWR, libc::O_NOATIME, false, libc::ENOENT),
        open_ne_47: (libc::O_RDWR, libc::O_NOCTTY, false, libc::ENOENT),
        open_ne_48: (libc::O_RDWR, libc::O_NDELAY, false, libc::ENOENT),
        open_ne_49: (libc::O_RDWR, libc::O_PATH, false, libc::ENOENT),
        open_ne_50: (libc::O_RDWR, libc::O_SYNC, false, libc::ENOENT),
        open_ne_51: (libc::O_RDWR, libc::O_TMPFILE, false, libc::ENOENT),
    }
}

pub mod common_open_creat {
    crate::open_creat! {
        //open_creat_01: (libc::O_RDONLY, libc::O_NONBLOCK, true, 0),
        //open_creat_02: (libc::O_RDONLY, libc::O_APPEND, true, 0),
        //open_creat_03: (libc::O_RDONLY, libc::O_CREAT, true, 0),
        //open_creat_04: (libc::O_RDONLY, libc::O_TRUNC, true, 0),
        //open_creat_05: (libc::O_RDONLY, libc::O_EXCL, true, 0),
        //open_creat_06: (libc::O_RDONLY, libc::O_NOFOLLOW, true, 0),
        //open_creat_07: (libc::O_RDONLY, libc::O_CLOEXEC, true, 0),
        open_creat_08: (libc::O_WRONLY, libc::O_NONBLOCK, true, 0),
        open_creat_09: (libc::O_WRONLY, libc::O_APPEND, true, 0),
        open_creat_10: (libc::O_WRONLY, libc::O_CREAT, true, 0),
        open_creat_11: (libc::O_WRONLY, libc::O_TRUNC, true, 0),
        open_creat_12: (libc::O_WRONLY, libc::O_EXCL, true, 0),
        open_creat_13: (libc::O_WRONLY, libc::O_NOFOLLOW, true, 0),
        open_creat_14: (libc::O_WRONLY, libc::O_CLOEXEC, true, 0),
        open_creat_15: (libc::O_RDWR, libc::O_NONBLOCK, true, 0),
        open_creat_16: (libc::O_RDWR, libc::O_APPEND, true, 0),
        open_creat_17: (libc::O_RDWR, libc::O_CREAT, true, 0),
        open_creat_18: (libc::O_RDWR, libc::O_TRUNC, true, 0),
        open_creat_19: (libc::O_RDWR, libc::O_EXCL, true, 0),
        open_creat_20: (libc::O_RDWR, libc::O_NOFOLLOW, true, 0),
        open_creat_21: (libc::O_RDWR, libc::O_CLOEXEC, true, 0),
    }
}

#[cfg(target_os = "linux")]
pub mod linux_open_creat {
    crate::open_creat! {
        open_creat_22: (libc::O_RDONLY, libc::O_ASYNC, true, 0),
        open_creat_23: (libc::O_RDONLY, libc::O_DIRECT, true, 0),
        open_creat_24: (libc::O_RDONLY, libc::O_DSYNC, true, 0),
        open_creat_25: (libc::O_RDONLY, libc::O_LARGEFILE, true, 0),
        open_creat_26: (libc::O_RDONLY, libc::O_NOATIME, true, 0),
        open_creat_27: (libc::O_RDONLY, libc::O_NOCTTY, true, 0),
        open_creat_28: (libc::O_RDONLY, libc::O_NDELAY, true, 0),
        open_creat_29: (libc::O_RDONLY, libc::O_PATH, false, libc::ENOENT),
        open_creat_30: (libc::O_RDONLY, libc::O_SYNC, true, 0),
        open_creat_31: (libc::O_RDONLY, libc::O_TMPFILE, false, libc::EINVAL),
        open_creat_32: (libc::O_WRONLY, libc::O_ASYNC, true, 0),
        open_creat_33: (libc::O_WRONLY, libc::O_DIRECT, true, 0),
        open_creat_34: (libc::O_WRONLY, libc::O_DSYNC, true, 0),
        open_creat_35: (libc::O_WRONLY, libc::O_LARGEFILE, true, 0),
        open_creat_36: (libc::O_WRONLY, libc::O_NOATIME, true, 0),
        open_creat_37: (libc::O_WRONLY, libc::O_NOCTTY, true, 0),
        open_creat_38: (libc::O_WRONLY, libc::O_NDELAY, true, 0),
        open_creat_39: (libc::O_WRONLY, libc::O_PATH, false, libc::ENOENT),
        open_creat_40: (libc::O_WRONLY, libc::O_SYNC, true, 0),
        open_creat_41: (libc::O_WRONLY, libc::O_TMPFILE, false, libc::EINVAL),
        open_creat_42: (libc::O_RDWR, libc::O_ASYNC, true, 0),
        open_creat_43: (libc::O_RDWR, libc::O_DIRECT, true, 0),
        open_creat_44: (libc::O_RDWR, libc::O_DSYNC, true, 0),
        open_creat_45: (libc::O_RDWR, libc::O_LARGEFILE, true, 0),
        open_creat_46: (libc::O_RDWR, libc::O_NOATIME, true, 0),
        open_creat_47: (libc::O_RDWR, libc::O_NOCTTY, true, 0),
        open_creat_48: (libc::O_RDWR, libc::O_NDELAY, true, 0),
        open_creat_49: (libc::O_RDWR, libc::O_PATH, false, libc::ENOENT),
        open_creat_50: (libc::O_RDWR, libc::O_SYNC, true, 0),
        open_creat_51: (libc::O_RDWR, libc::O_TMPFILE, false, libc::EINVAL),
    }
}

pub mod common_open_exist_ro {
    crate::open_exist_ro! {
        open_exist_ro_01: (libc::O_RDONLY, libc::O_NONBLOCK, true, 0),
        open_exist_ro_02: (libc::O_RDONLY, libc::O_APPEND, true, 0),
        open_exist_ro_03: (libc::O_RDONLY, libc::O_CREAT, true, 0),
        open_exist_ro_04: (libc::O_RDONLY, libc::O_TRUNC, false, libc::EACCES),
        open_exist_ro_05: (libc::O_RDONLY, libc::O_EXCL, true, 0),
        open_exist_ro_06: (libc::O_RDONLY, libc::O_NOFOLLOW, true, 0),
        open_exist_ro_07: (libc::O_RDONLY, libc::O_CLOEXEC, true, 0),
        open_exist_ro_08: (libc::O_WRONLY, libc::O_NONBLOCK, false, libc::EACCES),
        open_exist_ro_09: (libc::O_WRONLY, libc::O_APPEND, false, libc::EACCES),
        open_exist_ro_10: (libc::O_WRONLY, libc::O_CREAT, false, libc::EACCES),
        open_exist_ro_11: (libc::O_WRONLY, libc::O_TRUNC, false, libc::EACCES),
        open_exist_ro_12: (libc::O_WRONLY, libc::O_EXCL, false, libc::EACCES),
        open_exist_ro_13: (libc::O_WRONLY, libc::O_NOFOLLOW, false, libc::EACCES),
        open_exist_ro_14: (libc::O_WRONLY, libc::O_CLOEXEC, false, libc::EACCES),
        open_exist_ro_15: (libc::O_RDWR, libc::O_NONBLOCK, false, libc::EACCES),
        open_exist_ro_16: (libc::O_RDWR, libc::O_APPEND, false, libc::EACCES),
        open_exist_ro_17: (libc::O_RDWR, libc::O_CREAT, false, libc::EACCES),
        open_exist_ro_18: (libc::O_RDWR, libc::O_TRUNC, false, libc::EACCES),
        open_exist_ro_19: (libc::O_RDWR, libc::O_EXCL, false, libc::EACCES),
        open_exist_ro_20: (libc::O_RDWR, libc::O_NOFOLLOW, false, libc::EACCES),
        open_exist_ro_21: (libc::O_RDWR, libc::O_CLOEXEC, false, libc::EACCES),
    }
}

#[cfg(target_os = "linux")]
pub mod linux_open_exist_ro {
    crate::open_exist_ro! {
        open_exist_ro_22: (libc::O_RDONLY, libc::O_ASYNC, true, 0),
        open_exist_ro_23: (libc::O_RDONLY, libc::O_DIRECT, true, 0),
        open_exist_ro_24: (libc::O_RDONLY, libc::O_DSYNC, true, 0),
        open_exist_ro_25: (libc::O_RDONLY, libc::O_LARGEFILE, true, 0),
        open_exist_ro_26: (libc::O_RDONLY, libc::O_NOATIME, true, 0),
        open_exist_ro_27: (libc::O_RDONLY, libc::O_NOCTTY, true, 0),
        open_exist_ro_28: (libc::O_RDONLY, libc::O_NDELAY, true, 0),
        open_exist_ro_29: (libc::O_RDONLY, libc::O_PATH, true, 0),
        open_exist_ro_30: (libc::O_RDONLY, libc::O_SYNC, true, 0),
        open_exist_ro_31: (libc::O_RDONLY, libc::O_TMPFILE, false, libc::EINVAL),
        open_exist_ro_32: (libc::O_WRONLY, libc::O_ASYNC, false, libc::EACCES),
        open_exist_ro_33: (libc::O_WRONLY, libc::O_DIRECT, false, libc::EACCES),
        open_exist_ro_34: (libc::O_WRONLY, libc::O_DSYNC, false, libc::EACCES),
        open_exist_ro_35: (libc::O_WRONLY, libc::O_LARGEFILE, false, libc::EACCES),
        open_exist_ro_36: (libc::O_WRONLY, libc::O_NOATIME, false, libc::EACCES),
        open_exist_ro_37: (libc::O_WRONLY, libc::O_NOCTTY, false, libc::EACCES),
        open_exist_ro_38: (libc::O_WRONLY, libc::O_NDELAY, false, libc::EACCES),
        open_exist_ro_39: (libc::O_WRONLY, libc::O_PATH, true, 0),
        open_exist_ro_40: (libc::O_WRONLY, libc::O_SYNC, false, libc::EACCES),
        open_exist_ro_41: (libc::O_WRONLY, libc::O_TMPFILE, false, libc::ENOTDIR),
        open_exist_ro_42: (libc::O_RDWR, libc::O_ASYNC, false, libc::EACCES),
        open_exist_ro_43: (libc::O_RDWR, libc::O_DIRECT, false, libc::EACCES),
        open_exist_ro_44: (libc::O_RDWR, libc::O_DSYNC, false, libc::EACCES),
        open_exist_ro_45: (libc::O_RDWR, libc::O_LARGEFILE, false, libc::EACCES),
        open_exist_ro_46: (libc::O_RDWR, libc::O_NOATIME, false, libc::EACCES),
        open_exist_ro_47: (libc::O_RDWR, libc::O_NOCTTY, false, libc::EACCES),
        open_exist_ro_48: (libc::O_RDWR, libc::O_NDELAY, false, libc::EACCES),
        open_exist_ro_49: (libc::O_RDWR, libc::O_PATH, true, 0),
        open_exist_ro_50: (libc::O_RDWR, libc::O_SYNC, false, libc::EACCES),
        open_exist_ro_51: (libc::O_RDWR, libc::O_TMPFILE, false, libc::ENOTDIR),
    }
}

pub mod common_open_exist_rw {
    crate::open_exist_rw! {
        open_exist_rw_01: (libc::O_RDONLY, libc::O_NONBLOCK, true, 0),
        open_exist_rw_02: (libc::O_RDONLY, libc::O_APPEND, true, 0),
        open_exist_rw_03: (libc::O_RDONLY, libc::O_CREAT, true, 0),
        open_exist_rw_04: (libc::O_RDONLY, libc::O_TRUNC, true, 0),
        open_exist_rw_05: (libc::O_RDONLY, libc::O_EXCL, true, 0),
        open_exist_rw_06: (libc::O_RDONLY, libc::O_NOFOLLOW, true, 0),
        open_exist_rw_07: (libc::O_RDONLY, libc::O_CLOEXEC, true, 0),
        open_exist_rw_08: (libc::O_WRONLY, libc::O_NONBLOCK, true, 0),
        open_exist_rw_09: (libc::O_WRONLY, libc::O_APPEND, true, 0),
        open_exist_rw_10: (libc::O_WRONLY, libc::O_CREAT, true, 0),
        open_exist_rw_11: (libc::O_WRONLY, libc::O_TRUNC, true, 0),
        open_exist_rw_12: (libc::O_WRONLY, libc::O_EXCL, true, 0),
        open_exist_rw_13: (libc::O_WRONLY, libc::O_NOFOLLOW, true, 0),
        open_exist_rw_14: (libc::O_WRONLY, libc::O_CLOEXEC, true, 0),
        open_exist_rw_15: (libc::O_RDWR, libc::O_NONBLOCK, true, 0),
        open_exist_rw_16: (libc::O_RDWR, libc::O_APPEND, true, 0),
        open_exist_rw_17: (libc::O_RDWR, libc::O_CREAT, true, 0),
        open_exist_rw_18: (libc::O_RDWR, libc::O_TRUNC, true, 0),
        open_exist_rw_19: (libc::O_RDWR, libc::O_EXCL, true, 0),
        open_exist_rw_20: (libc::O_RDWR, libc::O_NOFOLLOW, true, 0),
        open_exist_rw_21: (libc::O_RDWR, libc::O_CLOEXEC, true, 0),
    }
}

#[cfg(target_os = "linux")]
pub mod linux_open_exist_rw {
    crate::open_exist_rw! {
        open_exist_rw_22: (libc::O_RDONLY, libc::O_ASYNC, true, 0),
        open_exist_rw_23: (libc::O_RDONLY, libc::O_DIRECT, true, 0),
        open_exist_rw_24: (libc::O_RDONLY, libc::O_DSYNC, true, 0),
        open_exist_rw_25: (libc::O_RDONLY, libc::O_LARGEFILE, true, 0),
        open_exist_rw_26: (libc::O_RDONLY, libc::O_NOATIME, true, 0),
        open_exist_rw_27: (libc::O_RDONLY, libc::O_NOCTTY, true, 0),
        open_exist_rw_28: (libc::O_RDONLY, libc::O_NDELAY, true, 0),
        open_exist_rw_29: (libc::O_RDONLY, libc::O_PATH, true, 0),
        open_exist_rw_30: (libc::O_RDONLY, libc::O_SYNC, true, 0),
        open_exist_rw_31: (libc::O_RDONLY, libc::O_TMPFILE, false, libc::EINVAL),
        open_exist_rw_32: (libc::O_WRONLY, libc::O_ASYNC, true, 0),
        open_exist_rw_33: (libc::O_WRONLY, libc::O_DIRECT, true, 0),
        open_exist_rw_34: (libc::O_WRONLY, libc::O_DSYNC, true, 0),
        open_exist_rw_35: (libc::O_WRONLY, libc::O_LARGEFILE, true, 0),
        open_exist_rw_36: (libc::O_WRONLY, libc::O_NOATIME, true, 0),
        open_exist_rw_37: (libc::O_WRONLY, libc::O_NOCTTY, true, 0),
        open_exist_rw_38: (libc::O_WRONLY, libc::O_NDELAY, true, 0),
        open_exist_rw_39: (libc::O_WRONLY, libc::O_PATH, true, 0),
        open_exist_rw_40: (libc::O_WRONLY, libc::O_SYNC, true, 0),
        open_exist_rw_41: (libc::O_WRONLY, libc::O_TMPFILE, false, libc::ENOTDIR),
        open_exist_rw_42: (libc::O_RDWR, libc::O_ASYNC, true, 0),
        open_exist_rw_43: (libc::O_RDWR, libc::O_DIRECT, true, 0),
        open_exist_rw_44: (libc::O_RDWR, libc::O_DSYNC, true, 0),
        open_exist_rw_45: (libc::O_RDWR, libc::O_LARGEFILE, true, 0),
        open_exist_rw_46: (libc::O_RDWR, libc::O_NOATIME, true, 0),
        open_exist_rw_47: (libc::O_RDWR, libc::O_NOCTTY, true, 0),
        open_exist_rw_48: (libc::O_RDWR, libc::O_NDELAY, true, 0),
        open_exist_rw_49: (libc::O_RDWR, libc::O_PATH, true, 0),
        open_exist_rw_50: (libc::O_RDWR, libc::O_SYNC, true, 0),
        open_exist_rw_51: (libc::O_RDWR, libc::O_TMPFILE, false, libc::ENOTDIR),
    }
}

/// open_01: Open new file with O_CREAT | O_EXCL
#[test]
fn open_01() {
    let mut path = crate::test_dir();
    path.push("open_01.txt");

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_RDWR | libc::O_CREAT | libc::O_EXCL,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert!(fd > 0);

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

/// open_02: Fail opening with O_CREAT | O_EXCL when the file already exists
#[test]
fn open_02() {
    let mut path = crate::test_dir();
    path.push("open_02.txt");

    crate::create_file(&mut path, &[]);

    let err =
        unsafe { libc::chmod(path.c_str(), libc::S_IRUSR | libc::S_IWUSR) };
    assert_eq!(err, 0);

    let fd = unsafe {
        crate::wrappers::open3(
            path.c_str(),
            libc::O_RDWR | libc::O_CREAT | libc::O_EXCL,
            libc::S_IRUSR | libc::S_IWUSR,
        )
    };
    assert_eq!(fd, -1);
    assert_eq!(crate::errno(), libc::EEXIST);
}
