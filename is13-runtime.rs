use bytes::Bytes;
use std::io::Read;

fn main() {
    let args: Vec<Bytes> = std::env::args().map(|a| a.into()).collect();

    let mut file = std::fs::File::open("examples/is13").unwrap();
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

