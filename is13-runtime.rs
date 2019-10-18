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
    let r = run_vm::<u32, SparseMemory<u32>>(&buffer, &[num]).e()?;
    match r {
        1 => println!("{:?} is not thirteen", opts.num),
        0 => println!("{:?} is thirteen", opts.num),
        _ => panic!(""),
    }

    Ok(())
}

use ckb_vm::{Register, Memory, TraceMachine, DefaultMachine, DefaultCoreMachine, WXorXMemory};

pub fn run_vm<R: Register, M: Memory<R> + Default>(
    program: &Bytes,
    args: &[Bytes],
) -> BResult<i8> {
    let mut machine =
        TraceMachine::new(DefaultMachine::<DefaultCoreMachine<R, WXorXMemory<R, M>>>::default());
    machine.load_program(program, args).e()?;
    Ok(machine.run().e()?)
}
