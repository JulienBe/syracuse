https://twitter.com/Adam_prepa/status/1503061876778885126

```rust
if current & 1 == 0 {                            // % 2 adds about 50% overall
    current = current >> 1;                      // / 2 adds about 100-120% overall
} else {
    current = (current * 3) + 1;
 // current = (current << 1) + current + 1;      // its a tadd slower
 // current = current + current + current + 1;   // about 20% slower
}
```
