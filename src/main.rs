extern crate clap;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use clap::{App, Arg};
extern crate ulid;
use ulid::Ulid;
use std::io::BufWriter;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

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
    let matches = App::new("ymlq")
        .about("Parses an input yaml and output v1.2 yaml file
usage:
    ymlfxr bad.yaml > good.yaml")
        .version("0.2.1")
        .author("Brett Smith <bc.smith@sas.com>")
        .arg(
            Arg::with_name("inplace")
                .help("Fix the file in place")
                .short("i")
                .long("fix")
        )
        .arg(
            Arg::with_name("backup")
                .help("Create backup of file")
                .short("b")
                .long("bak")
        )
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
    let backup = matches.is_present("backup");
    let _inplace = matches.is_present("inplace");
    let debug = matches.is_present("debug");

	let id = Ulid::new().to_string();
	if debug {
		println!("Job ID: {}", id);
	}
    if backup {
        let filebackup = format!("{}.bak", filename);
		println!("backing up {} as {}", &filename, &filebackup);
        fs::copy(&filename, &filebackup).expect("Unable to copy file");
    }

    let mut f = File::open(&filename).expect("file not found");

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
<<<<<<< HEAD
        let mut content = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut content);
            emitter.dump(doc).unwrap(); // dump the YAML object to a String
        }
        content.push_str("\n");
        buffer.write_all(content.as_bytes()).expect("unable to write bytes");
        buffer.flush().expect("unable to flush");
      }

      if _inplace {
		  if debug {
			  println!("renaming {} to {}", tempfile.display(), &filename);
		  }
          std::fs::rename(&tempfile, &filename).unwrap_or_else(|_x| {
          	// fs::rename does not support cross-device linking
          	// copy and delete instead
          	std::fs::copy(&tempfile, &filename).unwrap_or_else(|e| {
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
        tf.read_to_string(&mut output).expect("unable to read tempfile");
		trim_newline(&mut output);
	    println!("{}", output);
		std::fs::remove_file(&tempfile).unwrap_or_else(|e| {
		  exit_with_exception!(e, "failed to delete temporary file");
		});
  }
=======
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(doc).unwrap(); // dump the YAML object to a String
        }
        println!("{}", out_str);
    }
>>>>>>> 941907a1da567cb59ca51a0a28bb5dccfd601fb8
}
