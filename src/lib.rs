use std::path;
use std::sync;

#[cfg(test)]
mod tests;

const TEST_ROOT: &str = "./mountpoint";
static TEST_PATH: sync::OnceLock<path::PathBuf> = sync::OnceLock::new();

#[cfg(test)]
pub mod enums;

pub mod wrappers;

#[derive(Clone)]
pub struct TestPath {
    path: path::PathBuf,
    bytes: Vec<u8>,
}

impl TestPath {
    pub fn c_str(&mut self) -> *const libc::c_char {
        // Ensure that our paths are null-terminated
        let rsbytes = self.path.as_os_str().as_encoded_bytes();
        self.bytes = Vec::from(rsbytes);
        self.bytes.push(0);

        self.bytes.as_ptr() as *const libc::c_char
    }

    pub fn pop(&mut self) -> bool {
        self.path.pop()
    }

    pub fn push<P: AsRef<path::Path>>(&mut self, path: P) {
        self.path.push(path)
    }
}

impl From<path::PathBuf> for TestPath {
    fn from(path: path::PathBuf) -> Self {
        Self {
            path,
            bytes: Vec::new(),
        }
    }
}

pub fn test_dir() -> TestPath {
    let mut path = TEST_PATH
        .get_or_init(|| {
            let mut p = path::PathBuf::from(TEST_ROOT);
            p.push(rand_dir());
            p
        })
        .clone();

    path.push(rand_dir());
    std::fs::create_dir_all(&path).expect("Error creating test directory.");

    TestPath::from(path)
}

pub fn create_file(path: &mut TestPath, data: &[u8]) {
    let fd = unsafe {
        wrappers::open3(
            path.c_str(),
            libc::O_WRONLY | libc::O_CREAT,
            libc::S_IRUSR,
        )
    };
    assert!(fd > 0);

    let len = unsafe {
        libc::write(fd, data.as_ptr() as *const libc::c_void, data.len())
    };
    assert_eq!(len as usize, data.len());

    let err = unsafe { libc::close(fd) };
    assert_eq!(err, 0);
}

pub fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

pub fn perror(msg: &str) {
    unsafe { libc::perror(msg.as_ptr() as *const libc::c_char) }
    eprintln!();
}

fn rand_dir() -> String {
    use rand::prelude::*;

    let mut rng = rand::rng();

    (0..32)
        .map(|_| rng.sample(rand::distr::Alphanumeric) as char)
        .collect()
}
