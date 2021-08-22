use clap::{App, Arg};

use esp32::nvs::Nvs;
use esp32::VERSION;

fn main() {
    let app = App::new("nvs")
        .version(VERSION)
        .about("Host based tool for interacting with esp-idf nvs partitions")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Filename of the nvs partition")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("namespaces")
                .long("namespaces")
                .help("List all namespaces")
                .conflicts_with("keys")
                .conflicts_with("values")
                .conflicts_with("pairs")
                .conflicts_with("entries"),
        )
        .arg(
            Arg::with_name("namespace")
                .short("n")
                .long("namespace")
                .help("The namespace to interact with")
                .value_name("NS")
                .takes_value(true)
                .conflicts_with("namespaces"),
        )
        .arg(
            Arg::with_name("keys")
                .short("k")
                .long("keys")
                .help("Output only keys")
                .conflicts_with("namespaces")
                .conflicts_with("values")
                .conflicts_with("pairs")
                .conflicts_with("entries"),
        )
        .arg(
            Arg::with_name("values")
                .short("v")
                .long("values")
                .help("Output only values")
                .conflicts_with("namespaces")
                .conflicts_with("keys")
                .conflicts_with("pairs")
                .conflicts_with("entries"),
        )
        .arg(
            Arg::with_name("entries")
                .short("e")
                .long("entries")
                .help("Output full entry")
                .conflicts_with("namespaces")
                .conflicts_with("keys")
                .conflicts_with("values")
                .conflicts_with("pairs"),
        )
        .arg(
            Arg::with_name("pairs")
                .short("p")
                .long("pairs")
                .help("Output key value pairs")
                .conflicts_with("namespaces")
                .conflicts_with("keys")
                .conflicts_with("values")
                .conflicts_with("entries"),
        )
        .arg(
            Arg::with_name("deleted")
                .short("d")
                .long("deleted")
                .help("Interact with deleted entries"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Output format")
                .value_name("FORMAT")
                .possible_value("csv")
                .possible_value("json")
                .possible_value("text")
                .possible_value("yaml")
                .takes_value(true)
                .default_value("text"),
        )
        .get_matches();

    let file = app.value_of("file").unwrap();
    let nvs = Nvs::new(file, app.is_present("deleted"));

    let ns = app.value_of("namespace");

    if app.is_present("namespaces") {
        let namespaces = nvs.namespaces();
        namespaces.iter().for_each(|ns| {
            println!("{}", ns);
        });
    } else if app.is_present("keys") {
        if let Some(ns) = ns {
            let namespace = nvs.namespace(ns);
            if let Some(namespace) = namespace {
                namespace.keys().for_each(|key| println!("{}", key));
            }
            // TODO: should we let the user know the ns doesn't exist?
        } else {
            let entries = nvs.entries();
            entries.iter().for_each(|entry| println!("{}", entry.key()));
        };
    } else if app.is_present("values") {
        if let Some(ns) = ns {
            let namespace = nvs.namespace(ns);
            if let Some(namespace) = namespace {
                namespace
                    .values()
                    .for_each(|entry| println!("{}", entry.data()));
            }
            // TODO: should we let the user know the ns doesn't exist?
        } else {
            let entries = nvs.entries();
            entries
                .iter()
                .for_each(|entry| println!("{}", entry.data()));
        };
    } else if app.is_present("pairs") {
        if let Some(ns) = ns {
            let namespace = nvs.namespace(ns);
            if let Some(namespace) = namespace {
                namespace
                    .values()
                    .for_each(|entry| println!("{}: {}", entry.key(), entry.data()));
            }
            // TODO: should we let the user know the ns doesn't exist?
        } else {
            let entries = nvs.entries();
            entries
                .iter()
                .for_each(|entry| println!("{}: {}", entry.key(), entry.data()));
        };
    } else if app.is_present("entries") {
        if let Some(ns) = ns {
            let namespace = nvs.namespace(ns);
            if let Some(namespace) = namespace {
                namespace.values().for_each(|entry| println!("{:?}", entry));
            }
            // TODO: should we let the user know the ns doesn't exist?
        } else {
            let entries = nvs.entries();
            entries.iter().for_each(|entry| println!("{:?}", entry));
        };
    }
}
