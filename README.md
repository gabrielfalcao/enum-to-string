# enum-to-string (O.G.)

This crate provides the `EnumToString` derive macro.

The derive macro `EnumToString` implements the methods `variants()`,
`as_str()` and `as_dbg()` to enums with no associated values

`EnumToString` also the traits `std::fmt::Display` and
`std::fmt::Debug` by default and can switched on/off through the
`#[enum_to_string()]` attribute-like macro.


## Example

Turning off both `std::fmt::Display` and `std::fmt::Debug` trait
implementations.

```rust
use enum_to_string::EnumToString;
use enum_to_string::enum_to_string;


#[derive(EnumToString, Debug)]
#[enum_to_string(display = false, debug = false)]
enum Shell {
    Sh,
    Bash,
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}::{}",
            module_path!(),
            self.as_dbg()
        )
    }
}



fn main() {
    println!("{}", dbg!(Shell::Sh.as_str()));
    println!("{}", dbg!(Shell::Bash.as_str()));
    println!("{}", dbg!(Shell::Sh.as_dbg()));
    println!("{}", dbg!(Shell::Bash.as_dbg()));
    println!("{}", dbg!(format!("{}", Shell::Sh)));
    println!("{}", dbg!(format!("{}", Shell::Bash)));
    println!("{}", dbg!(format!("{:#?}", Shell::Sh)));
    println!("{}", dbg!(format!("{:#?}", Shell::Bash)));
}
```

You can find this [example](https://github.com/gabrielfalcao/enum-to-string/blob/main/examples/no-auto-display-debug-impl.rs) as well as other examples in the [examples](https://github.com/gabrielfalcao/enum-to-string/tree/main/examples) directory.
