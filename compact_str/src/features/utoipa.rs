use crate::CompactString;

#[cfg_attr(docsrs, doc(cfg(feature = "utoipa")))]
impl utoipa::PartialSchema for CompactString {
    #[inline]
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        std::string::String::schema()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "utoipa")))]
impl utoipa::ToSchema for CompactString {
    #[inline]
    fn name() -> std::borrow::Cow<'static, str> {
        std::string::String::name()
    }

    #[inline]
    fn schemas(
        schemas: &mut std::vec::Vec<(
            std::string::String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        std::string::String::schemas(schemas)
    }
}
