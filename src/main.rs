extern crate flate2;

use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
// use std::env::args;
use std::fs::File;

use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} `compress/decompress`  <input_file> <output_file>",
            args[0]
        );
        return;
    }

    let input_path = &args[2];
    let output_path = &args[3];

    let _input_file = match File::open(input_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening input file: {}", err);
            return;
        }
    };

    let input = BufReader::new(File::open(input_path).unwrap());
    let output = File::create(output_path).unwrap();

    if &args[1] == "compress" {
        compress(input, output)
    } else if &args[1] == "decompress" {
        decompress(input, output)
    } else {
        eprint!("invalid command \nsupported command compress and decompress");
    };
}

fn compress(mut input: BufReader<File>, output: File) {
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}

fn decompress(mut input: BufReader<File>, mut output: File) {
    let mut decoder = GzDecoder::new(&mut input);
    let start = Instant::now();
    let _ = copy(&mut decoder, &mut output).unwrap();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("Target len:{:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
