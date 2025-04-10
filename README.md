# bashhh

A quiet shell written in Rust, shhh!

## Development

First, make sure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then, you can build and run the project with the following commands:

```
cargo run
cargo test
cargo fmt
```

## Tests

```
✗ grep print < src/main.rs 
println!("> ");
```

## Roadmap

### 🚧 0.2.0

- Implement "Up" key history functionality  
- Implement tab completion  
- Maintain color support  
- Add autocompletion (TBD)

### ✅ 0.1.0

- Implement data structures  
    - Simple command structures  
    - Pipeline structures  
- Add tests  
    - Test pipelines  
    - Test simple commands  
- Execute basic commands  
    - Handle simple commands with input/output files  
- Add command parser  
- Implement `cd` and `exit` commands  
- Support pipes execution  
