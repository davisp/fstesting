// Compare the behavior of two filesystems by running a random set of "commands"
// against a read/write file on both filesystems and comparing the results.

use std::cmp::Eq;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};
use std::os::unix::fs::FileExt;
use std::path::PathBuf;
use std::sync::OnceLock;

use quickcheck::{Arbitrary, Gen, quickcheck};
use rand::RngCore;

static MAX_FILE_SIZE: OnceLock<usize> = OnceLock::new();

#[derive(Clone, Debug)]
struct BoundedUsize(usize);

impl Deref for BoundedUsize {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Arbitrary for BoundedUsize {
    fn arbitrary(g: &mut Gen) -> Self {
        BoundedUsize(usize::arbitrary(g) % MAX_FILE_SIZE.get().unwrap())
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let iter = self.0.shrink();
        Box::new(iter.map(Self))
    }
}

#[derive(Clone, Debug)]
enum Command {
    Reopen,
    Read(BoundedUsize),
    PRead(BoundedUsize, BoundedUsize),
    Write(BoundedUsize),
    PWrite(BoundedUsize, BoundedUsize),
    Seek(BoundedUsize),
    Truncate(BoundedUsize),
    Fsync,
    Size,
}

impl Command {
    fn apply(&self, fd1: &mut TestFile, fd2: &mut TestFile) -> Result<()> {
        match self {
            Self::Reopen => {
                let res1 = fd1.reopen();
                let res2 = fd2.reopen();

                self.check_res(res1, res2)?;
            }
            Self::Read(count) => {
                let mut bytes1 = vec![0u8; count.0];
                let mut bytes2 = vec![0u8; count.0];
                let res1 = fd1.read(&mut bytes1[..]);
                let res2 = fd2.read(&mut bytes2[..]);

                self.check_res(res1, res2)?;
                self.check_bytes(bytes1, bytes2)?;
            }
            Self::PRead(offset, count) => {
                let mut bytes1 = vec![0u8; count.0];
                let mut bytes2 = vec![0u8; count.0];
                let res1 = fd1.read_at(&mut bytes1, offset.0 as u64);
                let res2 = fd2.read_at(&mut bytes2, offset.0 as u64);

                self.check_res(res1, res2)?;
                self.check_bytes(bytes1, bytes2)?;
            }
            Self::Write(count) => {
                let mut bytes = vec![0u8; count.0];
                rand::rng().fill_bytes(&mut bytes);

                let res1 = fd1.write(&bytes[..]);
                let res2 = fd2.write(&bytes[..]);

                self.check_res(res1, res2)?;
            }
            Self::PWrite(offset, count) => {
                let mut bytes = vec![0u8; count.0];
                rand::rng().fill_bytes(&mut bytes);

                let res1 = fd1.write_at(&bytes[..], offset.0 as u64);
                let res2 = fd2.write_at(&bytes[..], offset.0 as u64);

                self.check_res(res1, res2)?;
            }
            Self::Seek(offset) => {
                let pos = SeekFrom::Start(offset.0 as u64);
                let res1 = fd1.seek(pos);
                let res2 = fd2.seek(pos);

                self.check_res(res1, res2)?;
            }
            Self::Truncate(offset) => {
                let res1 = fd1.set_len(offset.0 as u64);
                let res2 = fd2.set_len(offset.0 as u64);

                self.check_res(res1, res2)?;
            }
            Self::Fsync => {
                let res1 = fd1.sync_all();
                let res2 = fd2.sync_all();

                self.check_res(res1, res2)?;
            }
            Self::Size => {
                let res1 = TestFile::size(fd1);
                let res2 = TestFile::size(fd2);

                self.check_res(res1, res2)?;
            }
        }

        Ok(())
    }

    fn check_res<T: Eq + Debug>(
        &self,
        res1: Result<T>,
        res2: Result<T>,
    ) -> Result<()> {
        if res1.is_ok() && res2.is_ok() {
            assert_eq!(res1.ok().unwrap(), res2.ok().unwrap());
        } else if res1.is_err() && res2.is_err() {
            assert_eq!(res1.err().unwrap().kind(), res2.err().unwrap().kind());
        } else {
            panic!("Oh noes");
        }

        Ok(())
    }

    fn check_bytes(&self, bytes1: Vec<u8>, bytes2: Vec<u8>) -> Result<()> {
        if bytes1 != bytes2 {
            for (usize, (b1, b2)) in
                bytes1.iter().zip(bytes2.iter()).enumerate()
            {
                if b1 == b2 {
                    continue;
                }

                return Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "Bytes read do not match, first difference at buffer offset {usize}: {b1} != {b2}"
                    ),
                ));
            }
        }

        Ok(())
    }
}

impl Arbitrary for Command {
    fn arbitrary(g: &mut Gen) -> Self {
        match usize::arbitrary(g) % 8 {
            0 => Command::Reopen,
            1 => Command::Read(BoundedUsize::arbitrary(g)),
            2 => Command::PRead(
                BoundedUsize::arbitrary(g),
                BoundedUsize::arbitrary(g),
            ),
            3 => Command::Write(BoundedUsize::arbitrary(g)),
            4 => Command::PWrite(
                BoundedUsize::arbitrary(g),
                BoundedUsize::arbitrary(g),
            ),
            5 => Command::Seek(BoundedUsize::arbitrary(g)),
            6 => Command::Truncate(BoundedUsize::arbitrary(g)),
            7 => Command::Fsync,
            _ => Command::Size,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            Command::Reopen => Box::new((0..1).map(|_| Command::Reopen)),
            Command::Read(count) => {
                let i = count.shrink();
                Box::new(i.map(Command::Read))
            }
            Command::PRead(offset, count) => {
                let i1 = offset.shrink();
                let i2 = count.shrink();
                Box::new(i1.zip(i2).map(|(o, c)| Command::PRead(o, c)))
            }
            Command::Write(count) => {
                let i = count.shrink();
                Box::new(i.map(Command::Write))
            }
            Command::PWrite(offset, count) => {
                let i1 = offset.shrink();
                let i2 = count.shrink();
                Box::new(i1.zip(i2).map(|(o, c)| Command::PRead(o, c)))
            }
            Command::Seek(pos) => Box::new(pos.shrink().map(Command::Seek)),
            Command::Truncate(count) => {
                Box::new(count.shrink().map(Command::Truncate))
            }
            Command::Fsync => Box::new((0..1).map(|_| Command::Fsync)),
            Command::Size => Box::new((0..1).map(|_| Command::Size)),
        }
    }
}

struct TestFile {
    path: String,
    fd: File,
}

impl TestFile {
    fn create_new(dir: String, fname: String) -> Result<Self> {
        let path = PathBuf::from(dir).join(fname).display().to_string();
        let fd = std::fs::File::create_new(&path)?;
        Ok(Self { path, fd })
    }

    fn size(&self) -> Result<u64> {
        std::fs::metadata(&self.path).map(|md| md.len())
    }

    fn reopen(&mut self) -> Result<()> {
        self.fd = std::fs::File::open(&self.path)?;
        Ok(())
    }
}

impl Deref for TestFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.fd
    }
}

impl DerefMut for TestFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fd
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path).expect("Error removing temp file");
    }
}

struct Test {
    fd1: TestFile,
    fd2: TestFile,
}

impl Test {
    fn new(dir1: String, dir2: String) -> Result<Self> {
        let fname = format!("{}.bin", uuid::Uuid::now_v7());
        let fd1 = TestFile::create_new(dir1, fname.clone())?;
        let fd2 = TestFile::create_new(dir2, fname)?;
        Ok(Self { fd1, fd2 })
    }

    pub fn run(&mut self, commands: Vec<Command>) -> Result<()> {
        println!("Commands: {}", commands.len());

        for cmd in commands.iter() {
            eprintln!("{cmd:?}");
            cmd.apply(&mut self.fd1, &mut self.fd2)?;
        }

        Ok(())
    }
}

fn run_test(commands: Vec<Command>) -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 4);

    Test::new(args[1].clone(), args[2].clone())?.run(commands)
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
