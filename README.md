# shell

What to do:
- Start by building a basic shell that reads input from the user, executes simple commands,and handles basic piping or redirection.
- Expand the functionality to support background jobs, process control (like fg, bg, kill), and more advanced features like command history.

0.1.0
1. ✅ hacer el tad
    - ✅ simple command
    - ✅ pipeline
2. ✅ add tests
    - ✅ pipeline
    - ✅ simple command
3. ✅ Execute basic commands
    - ✅ simple command in and out files
4. ✅ Add parser
5. ✅ Execute cd and exit
6. ✅ Execute pipes
0.2.0
7. Make up key work
8. Make tab work
9. Keep colors
10. Autocomplete?

## Tests

```
✗ grep print < src/main.rs 
println!("> ");
```

## Roadmap

### 🚧 0.2.0

- 🟢 Implement "Up" key history functionality  
- Implement tab completion  
- Maintain color support  
- Add autocompletion (TBD)

#### Bugfix
- `cd` command has an error `/usr/bin/cd: line 4: cd: src: No such file or directory`.
- Crash when missing command.

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
