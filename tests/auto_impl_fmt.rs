use enum_to_string::*;


#[derive(EnumToString)]
pub enum Shell {
    Sh,
    Bash,
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_as_str() {
        assert_eq!(Shell::Sh.as_str(), "sh");
        assert_eq!(Shell::Bash.as_str(), "bash");
    }
    #[test]
    fn test_as_dbg() {
        assert_eq!(Shell::Sh.as_dbg(), "Shell::Sh");
        assert_eq!(Shell::Bash.as_dbg(), "Shell::Bash");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Shell::Sh), "sh");
        assert_eq!(format!("{}", Shell::Bash), "bash");
    }


}
