# hinty

See the [blog post](https://jeremyfwebb.ninja/2025/2025-MAY-20) for more info.

I was trying to answer the question, _does `std::hint::likely()` or `unlikely()` **really** do anything?_

```sh
cargo build --release
./gen_data.sh 10000000 # 10 million
cargo bench
./run_perf.sh
```

There are three binaries in here that do the testing, one that generates the data.

They do what you'd think:
```sh
cargo run --bin                                                               
Available binaries:
    baseline // no hints
    generate // makes data
    likely   // uses the 'likely' hint
    unlikely // uses the 'unlikely' hint
```

NOTE: this projects `./cargo/config.toml` contains adjustments to have `rustc` emit assembly. s

I recommend viewing the asmbely of the binaries after compilation, notably their  `is_fib` impl to inspect differences.

Doing so is easy with, for example if you're inspecting the `likely` binary:
```sh
cargo asm --bin likely 
```
If you don't have that tool `cargo install cargo-show-asm` will see the job done.

It outputs handy helpers so you can rerun it with exactly the function you want to look at:
```sh
# 21 "likely::is_fib" [370] is the line I want so,
cargo asm --bin likely 21

```
<details><summary>`cargo asm` Output</summary>

```asm
likely::is_fib:
Lfunc_begin24:
        .cfi_startproc
        cmp qword ptr [rdi + 24], 0
        je LBB24_1
        push rbp
        .cfi_def_cfa_offset 16
        .cfi_offset rbp, -16
        mov rbp, rsp
        .cfi_def_cfa_register rbp
        mov r8, qword ptr [rdi + 32]
        mov r10, qword ptr [rdi + 40]
        movabs rcx, 8317987319222330741
        xor rcx, r8
        movabs rax, 7237128888997146477
        xor rax, r10
        movabs rdx, 7816392313619706465
        xor rdx, r8
        movabs r9, 8387220255154660723
        xor r9, rsi
        xor r9, r10
        add rdx, r9
        rol r9, 16
        xor r9, rdx
        add rcx, rax
        rol rax, 13
        xor rax, rcx
        rol rcx, 32
        add rcx, r9
        mov r8, rcx
        xor r8, rsi
        add rdx, rax
        rol rax, 17
        xor rax, rdx
        rol rdx, 32
        rol r9, 21
        movabs r10, 576460752303423488
        xor rcx, r10
        xor rcx, r9
        add r8, rax
        rol rax, 13
        add rdx, rcx
        xor rax, r8
        rol rcx, 16
        xor rcx, rdx
        rol r8, 32
        add rdx, rax
        add r8, rcx
        rol rax, 17
        xor rax, rdx
        rol rcx, 21
        rol rdx, 32
        xor rcx, r8
        xor r8, r10
        xor rdx, 255
        add r8, rax
        add rdx, rcx
        rol rax, 13
        xor rax, r8
        rol rcx, 16
        rol r8, 32
        xor rcx, rdx
        add rdx, rax
        rol rax, 17
        add r8, rcx
        xor rax, rdx
        rol rcx, 21
        xor rcx, r8
        rol rdx, 32
        add r8, rax
        add rdx, rcx
        rol rax, 13
        xor rax, r8
        rol rcx, 16
        rol r8, 32
        xor rcx, rdx
        add rdx, rax
        rol rax, 17
        add r8, rcx
        xor rax, rdx
        rol rcx, 21
        xor rcx, r8
        rol rdx, 32
        add r8, rax
        add rdx, rcx
        rol rax, 13
        xor rax, r8
        rol rcx, 16
        xor rcx, rdx
        add rdx, rax
        rol rax, 17
        rol rcx, 21
        mov r8, rdx
        rol r8, 32
        xor rcx, rax
        xor r8, rcx
        xor r8, rdx
        mov rax, r8
        shr rax, 57
        mov rcx, qword ptr [rdi]
        mov rdx, qword ptr [rdi + 8]
        movd xmm0, eax
        pxor xmm1, xmm1
        pshufb xmm0, xmm1
        xor edi, edi
        pcmpeqd xmm1, xmm1
LBB24_4:
        and r8, rdx
        movdqu xmm2, xmmword ptr [rcx + r8]
        movdqa xmm3, xmm2
        pcmpeqb xmm3, xmm0
        pmovmskb r9d, xmm3
        test r9d, r9d
        je LBB24_7
LBB24_5:
        rep bsf eax, r9d
        add rax, r8
        and rax, rdx
        shl rax, 3
        mov r10, rcx
        sub r10, rax
        mov al, 1
        cmp rsi, qword ptr [r10 - 8]
        je LBB24_9
        lea eax, [r9 - 1]
        and ax, r9w
        mov r9d, eax
        jne LBB24_5
        .p2align        4
LBB24_7:
        pcmpeqb xmm2, xmm1
        pmovmskb eax, xmm2
        test eax, eax
        jne LBB24_8
        lea r8, [r8 + rdi + 16]
        add rdi, 16
        jmp LBB24_4
LBB24_1:
        xor eax, eax
        ret
LBB24_8:
        xor eax, eax
LBB24_9:
        pop rbp
        ret
```

</details>