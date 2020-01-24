# cargo_reg_local

Find data from crates.io registry index in local cache.  
For now on linux only. The folder of the cache is this:  
`~\.cargo\registry\index\github.com-1ecc6299db9ec823\.cache\`  

The only argument is a crate name or a substring of the crate name.  

The CLI returns:  

1. a list of versions for a given crate_name  
2. all the crate_names (and last version) that contain the given substring  

## error handling with anyhow

Rust error handling has so much potential.  
But sadly the vanilla approach is so complicated.  
They still advocate that wrap() and expect() and panic() are good. Crazy.  
I must use the crate `anyhow` to achieve a normal simple error handling.  
But the namings in Rust are terrible. Even more terrible than microsoft's.  
What can you think when you hear the method name "context"?  
Or "expect" or "unwrap". Totally non-intuitive at all. Ahhhh :-(.  
I like the idea of the name "unwrap_or_panic()", but who will change the language now.  

## Build and run

```bash
clear; cargo make dev
```

and then use the example of how to run it in the last 4th line on the screen.  
Something like this:  

```bash
target/debug/cargo_reg_local thread
```
