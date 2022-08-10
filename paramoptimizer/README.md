## Setup:  
1. Take the dataset from [link](https://discord.com/channels/689979228841836632/692200535348215839/992829495834857522)  
2. put it in the root directory of this crate  
so it looks like this:  
```
parameter_optimizer/
├─ src/
├─ two_snake_snakedump.sqlite
├─ Cargo.lock
├─ Cargo.toml
```
3. run `cargo run --release`  
## Warning:  
by default this uses rayon, so expect high cpu usage.  
  
## Usage:  
in `src/main.rs` at line 109, you can input the values for your eval function. the current optimized values are stored there.  
  
## Algorithm:  
It runs a local search starting from the given values and minimizes the mean squared error across the dataset.  