# 2020-7-3

我又回来了！

## Hello world

sturct 和 c++中的 struct 是很像的，然后 rust 中似乎没有 class

支持以如下形式初始化，可以某个已有的 struct 示例继承？一些参数过来

```rust
// Instantiate a `Point`
let point: Point = Point { x: 10.3, y: 0.4 };

// Access the fields of the point
println!("point coordinates: ({}, {})", point.x, point.y);

// Make a new point by using struct update syntax to use the fields of our
// other one
let bottom_right = Point { x: 5.2, ..point };
```

struct 的解构赋值不是用 move 的。

Enum 的定义没有什么特别的

```rust
use crate::Status::{Poor, Rich};
// Automatically `use` each name inside `Work`.
use crate::Work::*;
```

可以通过 use 来减少 scoping

以下语法类似于 golang 的 itoa，这个 enum 中的属性值从 0 递增。

```rust
// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}
```

加更了加更了

终于开始变量了！！

rust 没有使用变量赋值的概念，而是使用了变量绑定的概念。
而且 rust 中的变量默认是不可变的 233 需要在声明的时候加上 mut 关键字来声明这个变量是个可变的变量。

这个变量的 scope 的机制和 golang 中的很像，变量的作用域只在自己的 scope 中生效，小的 scope 中可以访问和 shadow 定义大的 scope 中已经声明了的变量。

把一个变量绑定为自身，可以在当前 scope 中锁定这个变量。（没有办法绑定成其他值，原理不知道 XD）
