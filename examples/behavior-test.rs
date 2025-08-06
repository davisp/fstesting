// Compare the behavior of two filesystems by running a random set of "commands"
// against a read/write file on both filesystems and comparing the results.

use std::io::{Error, ErrorKind, Result};

use quickcheck::quickcheck;

use fstesting::commands::{Command, CommandsTest, MAX_FILE_SIZE};

fn run_test(commands: Vec<Command>) -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 4);

    let ret =
        CommandsTest::new(args[1].clone(), args[2].clone())?.run(commands);
    match ret {
        Ok(_) => eprintln!("Success"),
        Err(_) => {
            eprintln!("\n***************");
            eprintln!("*   FAILURE   *");
            eprintln!("***************\n")
        }
    }
    ret
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

    quickcheck(run_test as fn(_) -> _);

    eprintln!("\nSuccess!");
    Ok(())
}
