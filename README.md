# `psd` or PAF sequence divergence

Calculate the mean per-sequence divergence across a PAF file, using the `de:f` field in the PAF file.

Given an input PAF file generated from minimap2 using a command something like this:

```console
minimap2 -cx asm20 -t 32 --cs --secondary=no ref.fa query.fa | sort -k6,6 -k8,8
```

Note the default implementation currently does not consider overlapping alignmnets. The `-i` flag does.

## Usage

```console
psd - Calculate the per-sequence divergence from a PAF file
Max Brown <max.carter-brown@aru.ac.uk>

USAGE:
  psd [-h] <PAF>

FLAGS:
  -h, --help            Prints help information
  -i, --individual      Print individual sequence divergence
                        values. These will be sorted by the
                        query input order.
ARGS:
  <PAF>                 Path to PAF file
```
