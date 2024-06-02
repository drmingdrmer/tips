https://www.yuanguohuo.com/2019/01/02/jemalloc-heap-profiling/


# jemalloc的heap profiling

 发表于 2019-01-02  更新于 2020-08-20  所属分类 [jemalloc](https://www.yuanguohuo.com/categories/jemalloc/)  阅读次数:  Valine: [0](https://www.yuanguohuo.com/2019/01/02/jemalloc-heap-profiling/#valine-comments)

使用jemalloc时，可以通过profiling机制来发现并定位内存泄漏(memory leak )。本文翻译自[原文](https://github.com/jemalloc/jemalloc/wiki/Use-Case%3A-Heap-Profiling)并增加了一些例子。



# 安装 (1)

这里我们编译安装jemalloc.5.10，注意在configure的时候添加了`--enable-prof`选项，这样才能打开profiling机制。下文中通过MALLOC_CONF设置的参数都依赖于次选项。

```
wget https://github.com/jemalloc/jemalloc/archive/5.1.0.tar.gz
tar zxvf 5.1.0.tar.gz
cd jemalloc-5.1.0/
./autogen.sh
./configure --prefix=/usr/local/jemalloc-5.1.0 --enable-prof
make
make install
```

# 程序退出时的内存分配状态 (2)

作为最简单的情形，我们可以在程序退出时，查看还有哪些分配但未释放的内存，它们通常是内存泄漏的重要线索。

```
#include <stdio.h>
#include <stdlib.h>

void do_something(size_t i)
{
  // Leak some memory.
  malloc(i * 1024);
}

void do_something_else(size_t i)
{
  // Leak some memory.
  malloc(i * 4096);
}

int main(int argc, char **argv)
{
  size_t i, sz;

  for (i = 0; i < 80; i++)
  {
    do_something(i);
  }

  for (i = 0; i < 40; i++)
  {
    do_something_else(i);
  }

  return (0);
}
```

然后编译。注意：我们的代码里没有include jemalloc的头文件，编译的时候也不需要链接jemalloc库。启动的时候通过`LD_PRELOAD`指定jemalloc库的路径就可以了。这是jemalloc方便使用的地方。当然也可以include jemalloc的头文件并链接jemalloc库来使用jemalloc的其他功能(见后文)。

```
# gcc test.c -o a.out  
```

## 程序退出时的泄漏 (2.1)

```
# MALLOC_CONF=prof_leak:true,lg_prof_sample:0,prof_final:true LD_PRELOAD=/usr/local/jemalloc-5.1.0/lib/libjemalloc.so.2  ./a.out
<jemalloc>: Leak approximation summary: ~6926352 bytes, ~120 objects, >= 2 contexts
<jemalloc>: Run jeprof on "jeprof.34447.0.f.heap" for leak detail
```

程序退出时报告了泄漏的大概情况，多少自己，多少对象，并产生了一个”jeprof.34447.0.f.heap”文件，其中包含了详细信息。

## 泄漏的详细信息 (2.2)

使用jemalloc提供的jeprof工具，可以方便的查看”jeprof.34447.0.f.heap”文件：

```
# /usr/local/jemalloc-5.1.0/bin/jeprof a.out jeprof.34447.0.f.heap
Using local file a.out.
Using local file jeprof.34447.0.f.heap.
Welcome to jeprof!  For help, type 'help'.
(jeprof) top
Total: 6.6 MB
     3.3  50.6%  50.6%      3.3  50.6% do_something
     3.3  49.4% 100.0%      3.3  49.4% do_something_else
     0.0   0.0% 100.0%      6.6 100.0% __libc_start_main
     0.0   0.0% 100.0%      6.6 100.0% _start
     0.0   0.0% 100.0%      6.6 100.0% main
(jeprof)
```

## 泄露代码的调用路径 (2.3)

jeprof工具也可以生成泄漏代码的调用路径图。

```
# /usr/local/jemalloc-5.1.0/bin/jeprof  --show_bytes --pdf a.out jeprof.34447.0.f.heap > a.pdf
```

# Heap Profiling (3)

有时候，我们不能终止程序来看程序退出时的状态，jemalloc提供了一些方法来获取程序运行时的内存分配情况。

## 每1MB dump一次 (3.1)

```
# export MALLOC_CONF="prof:true,lg_prof_interval:20"
# LD_PRELOAD=/usr/local/jemalloc-5.1.0/lib/libjemalloc.so.2  ./a.out
# ll
total 40
-rwxr-xr-x. 1 root root 8520 Jan  2 18:33 a.out
-rw-r--r--. 1 root root 3878 Jan  2 18:38 jeprof.34584.0.i0.heap
-rw-r--r--. 1 root root 3882 Jan  2 18:38 jeprof.34584.1.i1.heap
-rw-r--r--. 1 root root 3882 Jan  2 18:38 jeprof.34584.2.i2.heap
-rw-r--r--. 1 root root 4004 Jan  2 18:38 jeprof.34584.3.i3.heap
-rw-r--r--. 1 root root 4004 Jan  2 18:38 jeprof.34584.4.i4.heap
-rw-r--r--. 1 root root 4006 Jan  2 18:38 jeprof.34584.5.i5.heap
```

其中`lg_prof_interval:20`中的20表示1MB(2^20)，`prof:true`是打开profiling。运行程序时，每分配(大约)1MB就会dump产生一个文件。

```
# /usr/local/jemalloc-5.1.0/bin/jeprof a.out jeprof.34584.3.i3.heap
Using local file a.out.
Using local file jeprof.34584.3.i3.heap.
Welcome to jeprof!  For help, type 'help'.
(jeprof) top
Total: 5.8 MB
     4.8  81.8%  81.8%      4.8  81.8% do_something
     1.1  18.2% 100.0%      1.1  18.2% do_something_else
     0.0   0.0% 100.0%      5.8 100.0% __libc_start_main
     0.0   0.0% 100.0%      5.8 100.0% _start
     0.0   0.0% 100.0%      5.8 100.0% main
(jeprof) quit
```

jeprof工具不仅可以查看详细信息或者生成调用路径图(如上所示)，还可以用来比较两个dump(显示增量部分)：

```
# /usr/local/jemalloc-5.1.0/bin/jeprof a.out --base=jeprof.34584.2.i2.heap jeprof.34584.3.i3.heap
Using local file a.out.
Using local file jeprof.34584.3.i3.heap.
Welcome to jeprof!  For help, type 'help'.
(jeprof) top
Total: 1.6 MB
     1.1  66.2%  66.2%      1.1  66.2% do_something_else
     0.5  33.8% 100.0%      0.5  33.8% do_something
     0.0   0.0% 100.0%      1.6 100.0% __libc_start_main
     0.0   0.0% 100.0%      1.6 100.0% _start
     0.0   0.0% 100.0%      1.6 100.0% main
(jeprof)
```

其中`--base`指定比较的基础。如上例，dump jeprof.34584.3.i3.heap的时候，分配了5.8 MB内存，do_something和do_something_else分别占81.8%和18.2%；但和dump jeprof.34584.2.i2.heap的时候相比，多分配了1.6MB内存，do_something和do_something_else分别占66.2%和33.8%。可以预见，自己和自己比，没有内存被分配：

```
# /usr/local/jemalloc-5.1.0/bin/jeprof a.out --base=jeprof.34584.2.i2.heap jeprof.34584.2.i2.heap
Using local file a.out.
Using local file jeprof.34584.2.i2.heap.
Welcome to jeprof!  For help, type 'help'.
(jeprof) top
Total: 0.0 MB
(jeprof)
```

## 每次达到新高时dump (3.2)

```
# export MALLOC_CONF="prof:true,prof_gdump:true"
# LD_PRELOAD=/usr/local/jemalloc-5.1.0/lib/libjemalloc.so.2  ./a.out
```

## 在代码里手动dump (3.3)

注意：需要include jemalloc的头文件并链接jemalloc库。

```
#include <stdio.h>
#include <stdlib.h>
#include <jemalloc/jemalloc.h>

void do_something(size_t i)
{
  // Leak some memory.
  malloc(i * 1024);
}

void do_something_else(size_t i)
{
  // Leak some memory.
  malloc(i * 4096);
}

int main(int argc, char **argv)
{
  size_t i, sz;

  for (i = 0; i < 80; i++)
  {
    do_something(i);
  }

  mallctl("prof.dump", NULL, NULL, NULL, 0);

  for (i = 0; i < 40; i++)
  {
    do_something_else(i);
  }

  mallctl("prof.dump", NULL, NULL, NULL, 0);

  return (0);
}
```

编译(指定jemalloc头文件路径，并链接jemalloc库)：

```
# gcc -I/usr/local/jemalloc-5.1.0/include test.c -L/usr/local/jemalloc-5.1.0/lib -ljemalloc
```

然后设置MALLOC_CONF并执行程序：

```
# export MALLOC_CONF="prof:true,prof_prefix:jeprof.out"
# LD_PRELOAD=/usr/local/jemalloc-5.1.0/lib/libjemalloc.so.2  ./a.out
# ls
a.out  jeprof.out.35307.0.m0.heap  jeprof.out.35307.1.m1.heap
```

## 稳定状态的内存分配 (3.4)

注意：需要include jemalloc的头文件并链接jemalloc库。

程序启动的时候，势必要分配内存，我们查找内存泄漏的时候，往往更关注程序在稳定状态时的内存分配：只要程序启动完成之后内存不再增长，就没有严重的泄漏问题。所以，稳定状态的内存profiling往往更有意义。设置`MALLOC_CONF=prof_active:false`，使得程序在启动的时候profiling是disabled；程序启动完成后，再通过mallctl(“prof.active”)来enable profiling；或者定时enable。

### 启动完成后enable profiling (3.4.1)

```
#include <stdio.h>
#include <stdlib.h>
#include <jemalloc/jemalloc.h>

void do_something(size_t i)
{
  // Leak some memory.
  malloc(i * 1024);
}

void do_something_else(size_t i)
{
  // Leak some memory.
  malloc(i * 4096);
}

int main(int argc, char **argv)
{
  size_t i, sz;

  //initialization ...

  for (i = 0; i < 80; i++)
  {
    do_something(i);
  }

  //enter into steady-state...

  bool active = true;
  mallctl("prof.active", NULL, NULL, &active, sizeof(bool));

  for (i = 0; i < 40; i++)
  {
    do_something_else(i);
  }

  mallctl("prof.dump", NULL, NULL, NULL, 0);

  return (0);
}
```

编译，设置环境变量，并执行：

```
# gcc -I/usr/local/jemalloc-5.1.0/include test2.c -L/usr/local/jemalloc-5.1.0/lib -ljemalloc
# export MALLOC_CONF="prof:true,prof_active:false,prof_prefix:jeprof.out"
# LD_PRELOAD=/usr/local/jemalloc-5.1.0/lib/libjemalloc.so.2  ./a.out
# ls
a.out  jeprof.out.36842.0.m0.heap 
```

用jeprof查看，发现只有steady-state之后的内存分配：

```
# /usr/local/jemalloc-5.1.0/bin/jeprof a.out jeprof.out.36842.0.m0.heap
Using local file a.out.
Using local file jeprof.out.36842.0.m0.heap.
Welcome to jeprof!  For help, type 'help'.
(jeprof) top
Total: 2.8 MB
     2.8 100.0% 100.0%      2.8 100.0% do_something_else
     0.0   0.0% 100.0%      2.8 100.0% __libc_start_main
     0.0   0.0% 100.0%      2.8 100.0% _start
     0.0   0.0% 100.0%      2.8 100.0% main
(jeprof)
```

### 定时enable profiling (3.4.2)

还可以通过这样的流程定时dump：

```
bool active;

mallctl("prof.dump", NULL, NULL, NULL, 0);    //生成prof.1

active = true;
mallctl("prof.active", NULL, NULL, &active, sizeof(bool));

//sleep 30 seconds

active = false;
mallctl("prof.active", NULL, NULL, &active, sizeof(bool));

//sleep 30 seconds

mallctl("prof.dump", NULL, NULL, NULL, 0);   //生成prof.2
```

然后通过`jeprof a.out --base=prof.1 prof.2`来比较这两个dump，这可以突显出稳定状态下程序的内存分配行为。
