# Rust Practices

## 1. Rust Blogs
[一文讲透Rust中的PartialEq和Eq_Leigg原创](./blogs/about_eq_ord_trait.md)

## 2. Rust projects
### 2.1 Simple Shell
实现一个基本的shell，具备如下功能：

- 读取键盘输入，给出对应输出
- 分号隔开多条命令
- ~~命令Tab补齐、文件名补齐、- 上下键查看输入历史；~~（找不到合适的监听键盘输入的库）
- cd、ls、pwd等一些简单命令
- 显示/切换工作目录

演示：
```shell
lei@WilldeMacBook-Pro simpleshell % cargo run
Welcome to simpleshell-rs.
simpleshell >ls
Cargo.lock      Cargo.toml      src             target
simpleshell >pwd
/Users/lei/Desktop/Rust/rust_practices/simpleshell
simpleshell >cd ..
rust_practices >cd sim  
cd: No such file or directory (os error 2): sim
rust_practices >cd simpleshell
simpleshell >exit
lei@WilldeMacBook-Pro simpleshell %
```