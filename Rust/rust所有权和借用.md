## 1. 所有权与借用
```rust
// 所有权
// 模式：let v = 非引用声明。
// 变量 s 对堆上创建的 String 对象拥有所有权。
let s = String::from("hello world");

// 借用
// 模式: let v = 引用声明。
// 变量 s 指向了 字符串切片 &str (该值本身是引用)
let s = "hello world";
// 指针 b 引用了 变量 s
let b = &s; // b => &&str
// 这里 b 和 b.deref() 输出的值是相同的，因为 rust 本身会自动解引用，
// &T 切片类型本身实现了 deref() 解引用函数，所以 rust 会将 T 先解引用后再使用，也叫做 引用归一化 ：  &&&&T => &T   这两个是等价的
println!("{}, {}", b, b.deref());

```
