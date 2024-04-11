pub(crate) mod datetime_format {
    use sea_orm::prelude::DateTime;
    use serde::Serializer;

    pub fn serialize<S>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted_date = date.format("%Y-%m-%d %H:%M:%S").to_string();
        serializer.serialize_str(&formatted_date)
    }
}
