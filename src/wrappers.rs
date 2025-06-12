unsafe extern "C" {
    pub fn open3(
        path: *const libc::c_char,
        oflag: libc::c_int,
        mode: libc::mode_t,
    ) -> libc::c_int;
}
