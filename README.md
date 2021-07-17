# rust-llvm12-regression

This is a minimized test case for a rust-program that worked prior to upgrade to LLVM 12.

The program is the result of minimizing a much larger program. The program in its present state doesn't really make sense. But it probably shouldn't crash with a segfault.

The problem appears to be the glium macro "implement_buffer_content".

It does the following to calculate the size of a DST like this:

```rust
struct Data {
    data: [u32]
}

implement_buffer_content!(Data);
```

See docs:
https://docs.rs/glium/0.30.1/glium/macro.implement_buffer_content.html

Unfortunately, it then tries to figure out the minimum size and step by doing this:

```rust
use std::mem;

let fake_ptr: &$struct_name = unsafe { mem::transmute((0usize, 0usize)) };
let min_size = mem::size_of_val(fake_ptr);

let fake_ptr: &$struct_name = unsafe { mem::transmute((0usize, 1usize)) };
let step = mem::size_of_val(fake_ptr) - min_size;

size > min_size && (size - min_size) % step == 0
```

I believe the transmutes constitute undefined behaviour. Apparently, before LLVM 12
it worked. How can we salvage this?


