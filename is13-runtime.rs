use bytes::Bytes;
use std::io::Read;
use std::env;
use std::fs::File;
use ckb_vm::SparseMemory;
use b_error::{BResult, ResultExt};

fn main() {
    b_error::main(run)
}

fn run() -> BResult<()> {
    let args: Vec<Bytes> = env::args().map(|a| a.into()).collect();

    let mut file = File::open("is13-client").e()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).e()?;
    let buffer = Bytes::from(buffer);

    let r = ckb_vm::run::<u32, SparseMemory<u32>>(&buffer, &args[..]).e()?;
    match r {
        1 => println!("{:?} is not thirteen", args[1]),
        0 => println!("{:?} is thirteen", args[1]),
        _ => panic!(""),
    }

    Ok(())
}
