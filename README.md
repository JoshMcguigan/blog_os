# Blog OS

Implementation of the OS tutorials at [https://os.phil-opp.com/](https://os.phil-opp.com/).

## Build

```rust
bootimage build
```

## Boot

```rust
bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -display none
```
