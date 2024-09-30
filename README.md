# `psd` or PAF sequence divergence

Given an input PAF file generated from minimap2 using a command something like this:

```console
minimap2 -cx asm20 -t 32 --cs --secondary=no ref.fa query.fa | sort -k6,6 -k8,8
```

Note this implementation currently does not consider overlapping alignmnets, so it's pretty crude.

