# Rust Practices

## 1. Simple Shell
实现一个基本的shell，具备如下功能：

- 读取键盘输入，给出对应输出
- 支持命令格式：Command [-Options] Argument1 Argument2
- Shell提示符，如果当前用户为超级用户，提示符为“#”；其他用户的提示符均为$
- 分号隔开多条命令
- 命令Tab补齐、文件名补齐
- cd、ls、pwd
- 上下键查看输入历史；history命令（保留100条）
- export (ENV设置)