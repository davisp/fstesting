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

pub const FILE_ACCESS_MODES: &[i32] = &[
    libc::O_RDONLY,
    libc::O_WRONLY,
    libc::O_RDWR,
    //libc::O_SEARCH,
    //libc::O_EXEC,
];

pub const FILE_OPEN_OPTIONS: &[i32] = &[
    libc::O_NONBLOCK,
    libc::O_APPEND,
    libc::O_CREAT,
    libc::O_TRUNC,
    libc::O_EXCL,
    libc::O_SHLOCK,
    libc::O_EXLOCK,
    //libc::O_DIRECTORY,
    libc::O_NOFOLLOW,
    libc::O_SYMLINK,
    libc::O_EVTONLY,
    libc::O_CLOEXEC,
    libc::O_NOFOLLOW_ANY,
    #[cfg(target_os = "linux")]
    libc::O_RESOLVE_BENEATH,
];

pub const ACCESS_MODES: &[libc::c_int] =
    &[libc::R_OK, libc::W_OK, libc::X_OK, libc::F_OK];
