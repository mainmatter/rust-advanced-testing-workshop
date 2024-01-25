//! Introduce a redaction in the test in this exercise to replace the timestamp with a `[timestamp]` placeholder.
//! Do **not** update the saved snapshot.
#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;
    use serde_json::json;
    use time::format_description::well_known::Iso8601;

    #[test]
    fn snapshot() {
        let created_at = time::OffsetDateTime::now_utc()
            .format(&Iso8601::DEFAULT)
            .unwrap();
        let api_response = json!({
            "code": 201,
            "created_at": created_at,
            "payload": {
                "features": [
                    "serde",
                    "json"
                ]
            }
        });
        assert_json_snapshot!(api_response, {
            ".created_at" => "[timestamp]"
        })
    }
}
