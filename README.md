# ymlfxr

## description

rust cli for formating yaml v1.2 that passes yamllint
with default settings


## build

```bash
cargo build --release
```

## test

```bash
cargo run bad.yaml
```

output:

```bash
    Updating crates.io index
   Compiling libc v0.2.69
   Compiling bitflags v1.2.1
   Compiling unicode-width v0.1.7
   Compiling vec_map v0.8.1
   Compiling linked-hash-map v0.5.2
   Compiling strsim v0.8.0
   Compiling ansi_term v0.11.0
   Compiling textwrap v0.11.0
   Compiling yaml-rust v0.4.3
   Compiling atty v0.2.14
   Compiling clap v2.33.0
   Compiling ymlfxr v0.1.0 (/Users/bcsmit/go/src/github.com/xbcsmith/ymlfxr)
    Finished dev [unoptimized + debuginfo] target(s) in 10.32s
     Running `target/debug/ymlfxr bad.yaml`
---
name: foo
version: 0.1.0
release: ~
description: The foo package
summary: foo is the foo
requires:
  - bar
  - caz
provides:
  - foo
instructions:
  - unpack: tar -xvf foo-0.1.0.tar.xz && cd sharutils-0.1.0
    pre: "sed -i 's/bar/foo/g' Makefile.in"
    configure: "./configure --prefix=/usr"
    build: make
    test: make check
    install: make install
    post: ""
sources:
  - archive: foo-0.1.0.tar.xz
    md5: d3b07384d113edec49eaa6238ad5ff00
    sha256: b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c
    destination: /usr
```

```bash
yamllint bad.yaml
```

output:

```bash
bad.yaml
  1:1       warning  missing document start "---"  (document-start)
  7:1       error    wrong indentation: expected 2 but found 0  (indentation)
  10:1      error    wrong indentation: expected 2 but found 0  (indentation)
  12:1      error    wrong indentation: expected 2 but found 0  (indentation)
  20:1      error    wrong indentation: expected 2 but found 0  (indentation)
```

```bash
cargo run bad.yaml > good.yaml
```

```bash
yamllint good.yaml
```

no output!
