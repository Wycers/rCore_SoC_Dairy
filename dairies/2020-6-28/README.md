# 2020-6-28

## Hello world

编写了第一个 rust 程序

```rust
fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!")
}
```

第一印象是

- rust 的语法非常奇怪，在 println 后面需要一个"!"，不加会报错。
- rust 编译出来的可执行文件真的非常小巧！实现相同功能的程序，之前一直用作主力的 golang 因为自带 runtime，编译产物大于 2MB，而 rust 只要 200KB 不到。

为什么函数调用需要"!"？

"!"用于区分普通函数和宏(Macro)调用，好叭我还不知道 rust 中的宏调用是什么，但是看起来和 C 中的宏调用很不一样。

rust 使用 `fn` 关键字来声明和定义函数，`fn` 关键字隔一个空格后跟函数名，函数名后跟着一个括号，函数参数定义在括号内。

`fn main() {}` 是 rust 的入口函数。

rust 使用 snake_case 风格来命名函数，即所有字母小写并使用下划线类分隔单词，如：foo_bar。如果函数有返回值，则在括号后面加上箭头 -> ，在箭头后加上返回值的类型。

关于注释，其实和 C 系语言语言的差不多：

```plain
Regular comments which are ignored by the compiler:
// Line comments which go to the end of the line.
/* Block comments which go to the closing delimiter. */
Doc comments which are parsed into HTML library documentation:
/// Generate library docs for the following item.
//! Generate library docs for the enclosing item.
```

但是多了两种用于生成文档的注释的方式。不知道具体怎么用，应该不是重点。

```rust
let x = 5 + /* 90 + */ 5;
// ...
x = 10
```

按照很久以前从群友那吐得槽，试图对代码修改了一下，果然报错了。可能需要声明 x 为可变的。

这个格式化输出的系统非常强大，但是我不知道如何正常地输出"{0}"。。。

（好的 用`{{0}}`

这个`#[allow(dead_code)]`应该就是 rust 的宏调用了？看这个名字应该是允许无用声明的，有 go 那味了

迫不及待了！

变量可以重复声明，应该算是一个语法糖？
