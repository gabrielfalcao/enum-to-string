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
