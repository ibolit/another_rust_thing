use crate::api::Named;

pub fn name_is_in_named_vector<T: ?Sized + Named>(name: &str, haystack: &[&T]) -> bool {
    haystack
        .iter()
        .map(|x| x.name().clone())
        .collect::<Vec<String>>()
        .contains(&name.to_string())
}



#[cfg(test)]
mod tests {
    use super::*;

    struct MockNamed {
        name: String,
    }
    impl Named for MockNamed {
        fn name(&self) -> &String {
            &self.name
        }
    }

    #[test]
    fn test_name_is_in_named_vector() {
        let room = MockNamed {
            name: "Hello".to_owned(),
        };
        let named_vector = vec![&room];
        assert!(name_is_in_named_vector("Hello", &named_vector));
        assert!(!name_is_in_named_vector("Goodbye", &named_vector));
    }
}
