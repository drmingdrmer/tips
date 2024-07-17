lldb attach 进程后默认会拦截各种 signal，如果要调试 signal handle 的逻辑，需要设置 pass 放行。（gdb 同理，不过命令有些许不同

```
pro handle -p true -s false SIGINT
```
