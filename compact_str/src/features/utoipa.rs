use crate::CompactString;

#[cfg_attr(docsrs, doc(cfg(feature = "utoipa")))]
impl utoipa::__dev::ComposeSchema for CompactString {
    fn compose(
        new_generics: std::vec::Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        std::string::String::compose(new_generics)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_schema_type() {
        assert_eq!(
            utoipa::schema!(std::string::String),
            utoipa::schema!(crate::CompactString)
        );
    }
}
