
### 1. **Side-by-Side Colored Diff (Best for Terminal)**
```bash
diff -u --color=always baseline.s likely.s > diff_baseline_likely.diff
diff -u --color=always baseline.s unlikely.s > diff_baseline_unlikely.diff
diff -u --color=always likely.s unlikely.s > diff_likely_unlikely.diff

# View them with less (preserves colors)
less -R diff_*.diff
```

### 2. **Three-Way Diff (Best for Comprehensive Comparison)**
```bash
# Using diff3 (shows differences between all three files simultaneously)
diff3 baseline.s likely.s unlikely.s | less -R

# Or using vimdiff for a graphical side-by-side view
vimdiff baseline.s likely.s unlikely.s
```

### 3. **Focus Just on Branch Instructions**
```bash
# Compare just the branching-related instructions
grep -E 'jmp|je|jne|jl|jg|likely|unlikely' baseline.s > branches_baseline.s
grep -E 'jmp|je|jne|jl|jg|likely|unlikely' likely.s > branches_likely.s
grep -E 'jmp|je|jne|jl|jg|likely|unlikely' unlikely.s > branches_unlikely.s

# Then diff these filtered files
vimdiff branches_*.s
```

### 4. **Generate HTML Report (Great for Sharing)**
```bash
# Using diff2html for a beautiful web view
diff2html -i file -F diff_report.html -- baseline.s likely.s unlikely.s

# Then open in browser
xdg-open diff_report.html
```

### 5. **Key Things to Look For**
When comparing:
1. Search for `likely`/`unlikely` macros - these are the hints you added
2. Compare jump instructions (`jmp`, `je`, `jne` etc.) near hot loops
3. Look at function prologues/epilogues
4. Check for any reordering of basic blocks

### Pro Tip:
If the files are large, first filter to just the relevant functions:
```bash
# Extract just the fib function (adjust name if different)
awk '/fib:/ {p=1} p; /^\./ && p {p=0}' baseline.s > fib_baseline.s
awk '/fib:/ {p=1} p; /^\./ && p {p=0}' likely.s > fib_likely.s
awk '/fib:/ {p=1} p; /^\./ && p {p=0}' unlikely.s > fib_unlikely.s

# Then diff these focused files
vimdiff fib_*.s
```
