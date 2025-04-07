/// SPDX-FileCopyrightText: © 2018 Brett Smith <xbcsmith@gmail.com>
/// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};
use yaml_rust2::{YamlEmitter, YamlLoader};
extern crate ulid;
use ulid::Ulid;

macro_rules! exit_with_exception {
    ($error:ident, $extra:tt) => {
        let _ = write!(&mut std::io::stderr(), "{}\n", $extra);
        let _ = write!(&mut std::io::stderr(), "{}\n", $error);
        std::process::exit(-1);
    };
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
    let matches = Command::new("ymlfxr")
        .about(
            "Parses an input yaml and output v1.2 yaml file
usage:
    ymlfxr bad.yaml > good.yaml",
        )
        .version("0.3.2")
        .author("Brett Smith <bc.smith@sas.com>")
        .arg(
            Arg::new("inplace")
                .help("Fix the file in place")
                .short('i')
                .long("fix")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("backup")
                .help("Create backup of file")
                .short('b')
                .long("bak")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("multi")
            .help("Use multiline strings")
            .short('m')
            .long("multiline")
            .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("debug")
                .help("turn on debugging information")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.get_one::<String>("input").unwrap();
    let backup = matches.get_flag("backup");
    let _inplace = matches.get_flag("inplace");
    let multiline = matches.get_flag("multi");
    let debug = matches.get_flag("debug");

    let id = Ulid::new().to_string();
    if debug {
        println!("Job ID: {}", id);
    }
    if backup {
        let filebackup = format!("{}.bak", filename);
        println!("backing up {} as {}", &filename, &filebackup);
        fs::copy(filename, &filebackup).expect("Unable to copy file");
    }

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    let mut tempfile = PathBuf::from(&filename);
    tempfile.pop(); //now refers to parent, which might be nothing
    tempfile.push(id);
    if debug {
        println!("TempFile : {}", tempfile.display());
    }
    let mut buffer = BufWriter::new(File::create(&tempfile).expect("unable to create file"));

    // Multi document support, doc is a yaml::Yaml
    // let doc = &docs[0];
    for doc in docs.iter() {
        // Dump the YAML object
        let mut content = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut content);
            if multiline {
                emitter.multiline_strings(true);
            }
            emitter.dump(doc).unwrap(); // dump the YAML object to a String
        }
        content.push('\n');
        buffer
            .write_all(content.as_bytes())
            .expect("unable to write bytes");
        buffer.flush().expect("unable to flush");
    }

    if _inplace {
        if debug {
            println!("renaming {} to {}", tempfile.display(), &filename);
        }
        std::fs::rename(&tempfile, filename).unwrap_or_else(|_x| {
            // fs::rename does not support cross-device linking
            // copy and delete instead
            std::fs::copy(&tempfile, filename).unwrap_or_else(|e| {
                exit_with_exception!(e, "failed to fix file");
            });
            std::fs::remove_file(&tempfile).unwrap_or_else(|e| {
                exit_with_exception!(e, "failed to delete temporary file");
            });
        });
    } else {
        if debug {
            println!("printing to stdout\n");
        }
        let mut tf = File::open(&tempfile).expect("unable to open file");
        let mut output = String::new();
        tf.read_to_string(&mut output)
            .expect("unable to read tempfile");
        trim_newline(&mut output);
        println!("{}", output);
        std::fs::remove_file(&tempfile).unwrap_or_else(|e| {
            exit_with_exception!(e, "failed to delete temporary file");
        });
    }
}
