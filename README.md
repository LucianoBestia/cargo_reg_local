# cargo_reg_local

Find data from crates.io registry index in local cache.  
For now on linux only. The folder of the cache is this:  `~\.cargo\registry\index\github.com-1ecc6299db9ec823\.cache\`  

The only argument is a crate name or a substring of the crate name.  

The CLI returns:  

1. a list of versions for a given crate_name  
2. all the crate_names (and last version) that contain the given substring  

## Build and run

```bash
clear; cargo make dev
```

and then use the example how run it in the last 4th line. Like this:

```bash
target/debug/cargo_reg_local thread
```
