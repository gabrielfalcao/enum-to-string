use enum_to_string::*;

#[derive(EnumToString)]
pub enum Method {
    Get,
    Post,
    Delete,
    Put,
    Head,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(
            Method::variants(),
            vec![
                Method::Get,
                Method::Post,
                Method::Delete,
                Method::Put,
                Method::Head,
            ]
        );
    }
}
