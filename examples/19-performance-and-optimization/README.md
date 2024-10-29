# Performance Tips

- Always benchmark your code

- [Profile](https://nnethercote.github.io/perf-book/profiling.html) the hotspot parts of your system

- Use Box for Heap-Allocated Objects

- Avoid Unnecessary Cloning

- Reuse Memory with Object Pools

- Minimize Heap Allocation
  - Small allocations are not necessarily cheaper than large allocations.

- Use Static for Static Data

- Employ Cow for Efficient String Handling

- Optimize Data Structures

## Other tips and tricks

### I/O

- Use [Memory Mapped Files](https://docs.rs/memmap2/latest/memmap2/)

- [Do DMA](https://docs.rust-embedded.org/embedonomicon/dma.html#direct-memory-access-dma)
