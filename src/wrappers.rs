unsafe extern "C" {
    pub fn fcntl_int(fd: i32, cmd: i32, arg: i32) -> i32;

    pub fn open3(
        path: *const libc::c_char,
        oflag: libc::c_int,
        mode: libc::mode_t,
    ) -> libc::c_int;
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    pub fn fcntl_prealloc(fd: i32, cmd: i32, fs: *mut libc::fstore_t) -> i32;

    pub fn fcntl_punchhole(
        fd: i32,
        cmd: i32,
        fph: *mut libc::fpunchhole_t,
    ) -> i32;
}
