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

use quickcheck::{Arbitrary, Gen};
use rand::RngCore;

pub static MAX_FILE_SIZE: OnceLock<usize> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct BoundedUsize(usize);

impl BoundedUsize {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

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
pub enum Command {
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
    pub fn apply(&self, fd1: &mut TestFile, fd2: &mut TestFile) -> Result<()> {
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
                // Only check file sizes after an fsync
                let res1 = fd1.sync_all();
                let res2 = fd2.sync_all();

                self.check_res(res1, res2);

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
        match (res1, res2) {
            (Ok(v1), Ok(v2)) => {
                if v1 != v2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Ok result mismatch: {:?} != {:?}", v1, v2),
                    ));
                }
            }
            (Err(e1), Err(e2)) => {
                if e1.kind() != e2.kind() {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Err result mismatch: {:?} != {:?}", e1, e2),
                    ));
                }
            }
            (Ok(v1), Err(e2)) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Result mismatch: Ok({:?}) != Err({:?})", v1, e2),
                ));
            }
            (Err(e1), Ok(v2)) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Result mismatch: Err({:?}) != Ok({:?})", e1, v2),
                ));
            }
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

pub struct TestFile {
    path: String,
    fd: File,
}

impl TestFile {
    pub fn create_new(dir: String, fname: String) -> Result<Self> {
        let path = PathBuf::from(dir).join(fname).display().to_string();
        let fd = std::fs::File::create_new(&path)?;
        Ok(Self { path, fd })
    }

    pub fn size(&self) -> Result<u64> {
        std::fs::metadata(&self.path).map(|md| md.len())
    }

    pub fn reopen(&mut self) -> Result<()> {
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

pub struct CommandsTest {
    fd1: TestFile,
    fd2: TestFile,
}

impl CommandsTest {
    pub fn new(dir1: String, dir2: String) -> Result<Self> {
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
