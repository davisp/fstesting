// Re-run all captured regressions from behavior-test

use std::io::{Error, ErrorKind, Result};
use std::panic;

use fstesting::commands::{BoundedUsize, Command, CommandsTest, MAX_FILE_SIZE};

fn test_1() -> Result<()> {
    run_test(vec![
        Command::PWrite(BoundedUsize::new(2011037), BoundedUsize::new(2539667)),
        Command::PRead(BoundedUsize::new(1), BoundedUsize::new(5)),
    ])
}

fn test_2() -> Result<()> {
    run_test(vec![Command::Truncate(BoundedUsize::new(1))])
}

fn test_3() -> Result<()> {
    run_test(vec![
        Command::Truncate(BoundedUsize::new(2)),
        Command::PRead(BoundedUsize::new(1), BoundedUsize::new(1)),
    ])
}

fn test_4() -> Result<()> {
    run_test(vec![
        Command::Truncate(BoundedUsize::new(2532034)),
        Command::Write(BoundedUsize::new(2419266)),
        Command::Truncate(BoundedUsize::new(662889)),
        Command::Reopen,
        Command::PRead(BoundedUsize::new(796278), BoundedUsize::new(1411041)),
    ])
}

fn test_5() -> Result<()> {
    run_test(vec![
        Command::PWrite(BoundedUsize::new(2070909), BoundedUsize::new(849415)),
        Command::Write(BoundedUsize::new(812677)),
        Command::Read(BoundedUsize::new(2107648)),
    ])
}

fn test_6() -> Result<()> {
    run_test(vec![
        Command::PWrite(BoundedUsize::new(10), BoundedUsize::new(1)),
        Command::Size,
    ])
}

fn test_7() -> Result<()> {
    run_test(vec![
        Command::Write(BoundedUsize::new(241047)),
        Command::PWrite(BoundedUsize::new(467681), BoundedUsize::new(2466799)),
        Command::Size,
    ])
}

fn test_8() -> Result<()> {
    run_test(vec![
        Command::PWrite(BoundedUsize::new(200092), BoundedUsize::new(510702)),
        Command::Truncate(BoundedUsize::new(284452)),
        Command::PWrite(BoundedUsize::new(401548), BoundedUsize::new(515254)),
        Command::PRead(BoundedUsize::new(54964), BoundedUsize::new(515875)),
    ])
}

fn test_9() -> Result<()> {
    run_test(vec![
        Command::Seek(BoundedUsize::new(1)),
        Command::Write(BoundedUsize::new(1)),
        Command::Fsync,
        Command::PWrite(BoundedUsize::new(67813), BoundedUsize::new(236421)),
        Command::Size,
    ])
}

fn run_test(commands: Vec<Command>) -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 4);

    CommandsTest::new(args[1].clone(), args[2].clone())?.run(commands)
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("usage: {} DIR1 DIR2 MAX_FILE_SIZE_MB", args[0]),
        ));
    }

    let size = args[3].parse::<usize>().expect("Invalid maximum file size");

    // The division by two is because for a max file size, we could technically
    // generate a MAX_FILE_SIZE write at offset MAX_FILE_SIZE though that'd be
    // unlikely to reach that exactly.
    MAX_FILE_SIZE.get_or_init(|| (size * 1024 * 1024) / 2);

    test_1()?;
    test_2()?;
    test_3()?;
    test_4()?;
    test_5()?;
    test_6()?;
    test_7()?;
    test_8()?;
    test_9()?;

    eprintln!("\nSuccess!");
    Ok(())
}
