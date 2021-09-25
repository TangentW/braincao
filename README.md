# braincao
Brainfuck evaluator written in Rust.

---

```Rust

let brainfuck = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]
    >++.>+.+++++++..+++.>++.<<+++++++++++++++.>.
    +++.------.--------.>+.>.";

let res = run(brainfuck.chars(), &mut Stdio);

println!("{:?}", res);

```

Output:

```
Hello World!
Ok(())
```

