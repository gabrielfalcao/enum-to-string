use enum_to_string::EnumToString;


#[derive(EnumToString)]
enum Shell {
    Sh,
    Bash,
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
