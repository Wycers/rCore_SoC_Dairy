# 2020-7-3

我又回来了！

## Hello world


sturct和c++中的struct是很像的，然后rust中似乎没有class


支持以如下形式初始化，可以某个已有的struct示例继承？一些参数过来

``` rust
// Instantiate a `Point`
let point: Point = Point { x: 10.3, y: 0.4 };

// Access the fields of the point
println!("point coordinates: ({}, {})", point.x, point.y);

// Make a new point by using struct update syntax to use the fields of our
// other one
let bottom_right = Point { x: 5.2, ..point };
```

struct的解构赋值不是用move的。

Enum的定义没有什么特别的

``` rust
use crate::Status::{Poor, Rich};
// Automatically `use` each name inside `Work`.
use crate::Work::*;
```

可以通过use来减少scoping



以下语法类似于golang的itoa，这个enum中的属性值从0递增。
``` rust
// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}
```


