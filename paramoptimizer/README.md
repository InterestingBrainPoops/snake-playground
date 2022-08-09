# Setup:  
Take the dataset from [link](https://discord.com/channels/689979228841836632/692200535348215839/992829495834857522)  
put it in the root directory of this crate  
so it looks like this:  
```
parameter_optimizer/
├─ src
├─ snakedump.sqlite
├─ Cargo.lock
├─ Cargo.toml
```
then run `cargo run --release`  
by default this uses rayon, so expect high cpu usage.  
in `src/main.rs` at line 109, you can input the values for your eval function. the current optimized values are stored there.  
