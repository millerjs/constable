# Constable

Single file database for Rust.

## Docs

### Create a database

Databases can be created with four options (using the OpenOptions crate): 

- Write
- Read
- Create
- Append

```rust
let mut posts_file = OpenOptions::new()
    .write(true)
    .read(true)
    .create(true)
    .append(true)
    .open("posts.dat")
    .unwrap();
```
