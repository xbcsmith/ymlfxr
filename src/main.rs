extern crate clap;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use clap::{App, Arg};

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("ymlq")
        .about("Parses an input yaml and output v1.2 yaml file
usage:
    ymlfxr bad.yaml > good.yaml")
        .version("0.2.0")
        .author("Brett Smith <bc.smith@sas.com>")
        .arg(
            Arg::with_name("debug")
                .help("turn on debugging information")
                .short("d")
                .long("debug")
        )
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1)
        )
        .get_matches();


    let filename = matches.value_of("input").unwrap();

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    // Multi document support, doc is a yaml::Yaml
    // let doc = &docs[0];
    for doc in docs.iter() {
        // Dump the YAML object
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(doc).unwrap(); // dump the YAML object to a String
        }
        println!("{}", out_str);
    }
}
