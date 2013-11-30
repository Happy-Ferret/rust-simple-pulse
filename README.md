# pulseaudio simple client in Rust

You need to have pulseaudio installed in order to compile and run this.  

## Try it!
```
# compile
rustc rust_simple_pulse.rs

# run
./rust_simple_pulse file.wav
```
## further plans 
Well, I don't have any yet - since this was only used for testing the ffi feature
of rust. Maybe I will move the pulse C interface bits to another lib later.

## License 
Licensed under BSD (2-Clause), see LICENSE.txt

