use bytes::Bytes;
use std::io::Read;
use std::fs::File;
use ckb_vm::SparseMemory;
use b_error::{BResult, ResultExt};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    bin: PathBuf,
    num: String,
}

fn main() {
    b_error::main(run)
}

fn run() -> BResult<()> {
    let opts = Opts::from_args();

    let mut file = File::open(&opts.bin).e()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).e()?;
    let buffer = Bytes::from(buffer);

    let num = Bytes::from(opts.num.clone());
    let r = ckb_vm::run::<u32, SparseMemory<u32>>(&buffer, &[num]).e()?;
    match r {
        1 => println!("{:?} is not thirteen", opts.num),
        0 => println!("{:?} is thirteen", opts.num),
        _ => panic!(""),
    }

    Ok(())
}
