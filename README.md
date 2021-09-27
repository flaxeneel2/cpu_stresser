# Rust CPU stresser

This program stresses the CPU over an extended period of time to help test cooling capabilities.


### Flags:
```
-s <hashes>     The number of total hashes to do. (Higher takes longer) (default: 100000000)
-t <threads>    The number of threads to use. (defaults to all threads).
```


#### To build:
Requirements:
 - Rust 
 - Git (to git clone)


Run the following commands
```
git clone https://github.com/flaxeneel2/cpu_stresser
cargo build --release
```

This will make a file in targets/release/

Depending on your operating system, it will make a .exe or a linux executable (ELF) file