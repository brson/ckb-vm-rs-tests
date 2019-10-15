use bytes::Bytes;
use std::io::Read;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<Bytes> = env::args().map(|a| a.into()).collect();

    let mut file = File::open("is13-client").expect("open is13-client");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer = Bytes::from(buffer);

    let r = ckb_vm::run::<u32, ckb_vm::SparseMemory<u32>>(&buffer, &args[..]).unwrap();
    match r {
        1 => println!("{:?} is not thirteen", args[1]),
        0 => println!("{:?} is thirteen", args[1]),
        _ => panic!(""),
    }
}
