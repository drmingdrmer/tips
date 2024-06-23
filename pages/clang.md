- Generate assembly; for `x86_64`, in intel syntax: `clang -masm=intel -arch x86_64 -S c.c`
  Or: `clang++ -S -mllvm --x86-asm-syntax=intel test.cpp`
  https://stackoverflow.com/questions/10990018/how-to-generate-assembly-code-with-clang-in-intel-syntax
- Clang cross compilation:
  https://clang.llvm.org/docs/CrossCompilation.html