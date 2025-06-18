/// statfs_01: Check support for statfs
#[test]
fn statfs_01() {
    let mut path = crate::test_dir();

    let stats = unsafe {
        let mut stats: libc::statfs = std::mem::zeroed();
        let err = libc::statfs(path.c_str(), &mut stats);
        assert_eq!(err, 0);
        stats
    };

    assert!(stats.f_type > 0);
}
