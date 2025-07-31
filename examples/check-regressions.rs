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
        Command::PRead(BoundedUsize::new(1), BoundedUsize::new(1))
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

    eprintln!("\nSuccess!");
    Ok(())
}
