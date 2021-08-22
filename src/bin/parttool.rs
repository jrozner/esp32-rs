use std::fs::File;
use std::io::Write;

use clap::{App, Arg};

use esp32::partition_table::PartitionTable;
use esp32::VERSION;

fn main() {
    let app = App::new("parttool")
        .version(VERSION)
        .about("Host based tool for interacting with esp-idf partition table")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Filename of the partition table")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .help("format to output in")
                .takes_value(true)
                .possible_value("csv")
                .possible_value("bin")
                .default_value("csv")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("file to output to")
                .takes_value(true),
        )
        .get_matches();

    let partition_table = PartitionTable::from_file(app.value_of("file").unwrap());
    let partitions = partition_table.partitions();

    let out: Box<dyn Write> = match app.value_of("output") {
        Some(file) => Box::new(File::create(file).unwrap()),
        None => Box::new(std::io::stdout()),
    };

    match app.value_of("format") {
        Some("csv") => {
            let mut wrt = csv::Writer::from_writer(out);
            partitions.iter().for_each(|partition| wrt.serialize(partition).unwrap());
            wrt.flush().unwrap();
        },
        Some("bin") => unimplemented!(),
        _ => unreachable!(),
    }
}

// binary to csv
// csv to binary
// print partition table
