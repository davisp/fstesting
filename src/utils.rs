use crate::test_path::TestPath;
use crate::wrappers;

pub fn create_file(path: &mut TestPath, data: &[u8]) {
    create_file_impl(path, data, libc::S_IRUSR);
}

pub fn create_file_rw(path: &mut TestPath, data: &[u8]) {
    create_file_impl(path, data, libc::S_IRUSR | libc::S_IWUSR);
}

pub fn create_file_impl(path: &mut TestPath, data: &[u8], perms: libc::mode_t) {
    let fd = unsafe {
        wrappers::open3(path.c_str(), libc::O_WRONLY | libc::O_CREAT, perms)
    };
    assert!(fd > 0);

    let len = unsafe {
        libc::write(fd, data.as_ptr() as *const libc::c_void, data.len())
    };
    assert_eq!(len as usize, data.len());

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

pub fn read_file(path: &mut TestPath) -> String {
    let fd = unsafe { libc::open(path.c_str(), libc::O_RDONLY) };
    assert!(fd > 0);

    let mut buf = vec![0u8; 1024 * 1024];
    let mut idx = 0usize;

    loop {
        let len = buf.len() - idx;
        let nread = unsafe {
            libc::read(fd, buf[idx..].as_mut_ptr() as *mut libc::c_void, len)
        };

        assert!(nread >= 0);

        if nread < len as isize {
            buf.resize(idx + nread as usize, 0);
            break;
        }

        idx = buf.len();
        buf.resize(buf.len() + 1024 * 1024, 0);
    }

    String::from_utf8_lossy(&buf).to_string()
}

pub fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

pub fn perror(msg: &str) {
    unsafe { libc::perror(msg.as_ptr() as *const libc::c_char) }
    eprintln!();
}

pub fn file_size(path: &mut TestPath) -> usize {
    stat(path).st_size as usize
}

pub fn stat(path: &mut TestPath) -> libc::stat {
    unsafe {
        let mut stat: libc::stat = std::mem::zeroed();
        let err = libc::stat(path.c_str(), &mut stat);
        assert_eq!(err, 0);
        stat
    }
}

pub fn lstat(path: &mut TestPath) -> libc::stat {
    unsafe {
        let mut stat: libc::stat = std::mem::zeroed();
        let err = libc::lstat(path.c_str(), &mut stat);
        assert_eq!(err, 0);
        stat
    }
}

pub fn statfs(path: &mut TestPath) -> libc::statfs {
    unsafe {
        let mut stats: libc::statfs = std::mem::zeroed();
        let err = libc::statfs(path.c_str(), &mut stats);
        assert_eq!(err, 0);
        stats
    }
}
