pub trait TryDefault {
    type Error;
    fn try_default() -> Result<Self, Self::Error>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct TestStruct {
        value: i32,
    }

    impl TryDefault for TestStruct {
        type Error = String;

        fn try_default() -> Result<Self, Self::Error> {
            Ok(TestStruct { value: 0 })
        }
    }

    #[test]
    fn test_try_default_success() {
        let result = TestStruct::try_default();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, 0);
    }

    #[derive(Debug)]
    struct FailingStruct;

    impl TryDefault for FailingStruct {
        type Error = String;

        fn try_default() -> Result<Self, Self::Error> {
            Err("Failed to create default instance".to_string())
        }
    }

    #[test]
    fn test_try_default_failure() {
        let result = FailingStruct::try_default();
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "Failed to create default instance".to_string()
        );
    }
}
