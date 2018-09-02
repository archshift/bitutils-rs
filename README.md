# bitutils

## Providing bitfields and related functionality

### API

At the core of bitutils lies one macro, `bf!`.

#### Syntax:

```rust
bf!(BitfieldName[uXX] {
    field1: lower:upper, // e.g. field1: 0:3, which would encompass the least significant nibble
    field2: 4:6,
    field3: 7:8,
    field5: 9:9,
    // ...
});
```

#### Usage:
```rust
let mut bf = BitfieldName::new(0xFFFF);
bf.field1.get();
bf.field2.set(0x3);
bf.val &= 0xFF;
println!("{:x?}", bf);
```

Bitutils also provides the `bits!` and `bit!` macros.
```rust
let foo = bits!(0xFFFF, 0:3);
assert_eq!(0xFF, foo);

let bar = bit!(0xF, 7);
assert_eq!(1, bar);
```
