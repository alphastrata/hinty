➜  hinty git:(master) ✗ cargo asm --lib 8                                                                          master ✭ ✱ ◼
   Compiling hinty v0.1.0 (/Users/smak/Documents/hinty)
    Finished `release` profile [optimized + debuginfo] target(s) in 0.63s

        .globl  <hinty::NumberSet>::is_fib_unlikely
.set <hinty::NumberSet>::is_fib_unlikely, <hinty::NumberSet>::is_fib_likely
➜  hinty git:(master) ✗ cargo asm --lib 6                                                                          master ✭ ✱ ◼
    Finished `release` profile [optimized + debuginfo] target(s) in 0.09s

        .globl  <hinty::NumberSet>::is_fib
.set <hinty::NumberSet>::is_fib, <hinty::NumberSet>::is_fib_likely
➜  hinty git:(master) ✗ cargo asm --lib 7                                                                          master ✭ ✱ ◼
    Finished `release` profile [optimized + debuginfo] target(s) in 0.08s

        .globl  <hinty::NumberSet>::is_fib_likely
        .p2align        4
<hinty::NumberSet>::is_fib_likely:
Lfunc_begin4:
        .cfi_startproc
        push rbp
        .cfi_def_cfa_offset 16
        .cfi_offset rbp, -16
        mov rbp, rsp
        .cfi_def_cfa_register rbp
        add rdi, 24
        call <hashbrown::map::HashMap<u64, (), std::hash::random::RandomState>>::get_inner::<u64>
        test rax, rax
        setne al
        pop rbp
        ret


The compiler has identified they're all the same.