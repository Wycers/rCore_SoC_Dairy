# 2020-6-29

## Hello world

rust 的 tuple 可以存放不同类型的数据

可以使用<tuple 名>.<索引>访问数据

可以通过以下方法区分整型和 tuple

```rust
println!("one element tuple: {:?}", (5u32,));
println!("just an integer: {:?}", (5u32));

```

形似 js 的解构赋值 2333

```rust
let tuple = (1, "hello", 4.5, true);
let (a, b, c, d) = tuple;
```

加上这句可以规定输出 Matrix 的行为（`Obj.toString()`?）不过我还不知道为什么在`write!()`后面加上`;`会 gg

```rust
// Similarly, implement `Display` for `Matrix`
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}\n{}, {}\n", self.0, self.1, self.2, self.3)
    }
}
```
