unsafe extern "C" {
    pub fn fcntl_int(fd: i32, cmd: i32, arg: i32) -> i32;

    pub fn open3(
        path: *const libc::c_char,
        oflag: libc::c_int,
        mode: libc::mode_t,
    ) -> libc::c_int;
}
