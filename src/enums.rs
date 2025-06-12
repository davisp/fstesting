pub const FILE_PERMISSIONS: &[libc::mode_t] = &[
    libc::S_ISUID,
    libc::S_ISGID,
    libc::S_ISVTX,
    libc::S_IRUSR,
    libc::S_IWUSR,
    libc::S_IXUSR,
    libc::S_IRGRP,
    libc::S_IWGRP,
    libc::S_IXGRP,
    libc::S_IROTH,
    libc::S_IWOTH,
    libc::S_IXOTH,
];

pub const ACCESS_MODES: &[libc::c_int] =
    &[libc::R_OK, libc::W_OK, libc::X_OK, libc::F_OK];
