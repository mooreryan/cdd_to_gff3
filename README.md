# CDD to GFF3 Parser

Convert CDD batch search results to GFF3 format.

## Install

### From source

From GitHub, assuming you have [Rust installed](https://www.rust-lang.org/tools/install):

```
$ git clone https://github.com/mooreryan/cdd_to_gff3.git
$ cd cdd_to_gff3
$ cargo build --release
```

Then move or symlink the binary (`./target/release/cdd_to_gff3`) somewhere on your path:

```
$ ln -s $(pwd)/target/release/cdd_to_gff3 $HOME/bin/cdd_to_gff3
```

## Usage

Try it out on the test data!

```
$ cdd_to_gff3 --input test_files/cdd_hits.txt > test_files/cdd_hits.gff3
```

Now you could import that gff3 file into Geneious or whatever you like :)

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option. This program may not be copied, modified, or distributed except according to those terms.
