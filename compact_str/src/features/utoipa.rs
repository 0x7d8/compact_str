use crate::CompactString;

#[cfg_attr(docsrs, doc(cfg(feature = "utoipa")))]
impl utoipa::__dev::ComposeSchema for CompactString {
    fn compose(
        new_generics: std::vec::Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        str::compose(new_generics)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "utoipa")))]
impl utoipa::ToSchema for CompactString {
    #[inline]
    fn name() -> std::borrow::Cow<'static, str> {
        str::name()
    }

    #[inline]
    fn schemas(
        schemas: &mut std::vec::Vec<(
            std::string::String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        str::schemas(schemas)
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
        assert_eq!(utoipa::schema!(str), utoipa::schema!(crate::CompactString));
        assert_eq!(utoipa::schema!(&str), utoipa::schema!(crate::CompactString));
    }
}
