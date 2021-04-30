// REGULAR EXPRESSIONS
// #################################################################################################
#[cfg(test)]
mod tests {
    use regex::{Regex, RegexBuilder};

    #[test]
    fn regex_validate_password() {
        let re = Regex::new(r"^[a-zA-Z0-9@#$%^&+=*!~)(]{8,256}$").unwrap();
        // invalids
        assert!(!re.is_match("1234567"));
        assert!(!re.is_match(&"`".repeat(8)));
        assert!(!re.is_match(&"№".repeat(8)));
        assert!(!re.is_match(&" ".repeat(8)));
        assert!(!re.is_match(&"-".repeat(8)));
        assert!(!re.is_match(&"_".repeat(8)));
        assert!(!re.is_match(&":".repeat(8)));
        assert!(!re.is_match(&"'".repeat(8)));
        assert!(!re.is_match(&"\"".repeat(8)));
        assert!(!re.is_match(&",".repeat(8)));
        assert!(!re.is_match(&".".repeat(8)));
        assert!(!re.is_match(&"<".repeat(8)));
        assert!(!re.is_match(&">".repeat(8)));
        assert!(!re.is_match(&"?".repeat(8)));
        assert!(!re.is_match(&"/".repeat(8)));
        assert!(!re.is_match(&"  ".repeat(8)));
        assert!(!re.is_match(""));
        assert!(!re.is_match(&"0".repeat(257)));
        // valids
        assert!(re.is_match(&"@#$%^&+=*!~)("));
        assert!(re.is_match(&"0123456789"));
        assert!(re.is_match(&"abcdefghijklmnopqrstuvwxyz"));
        assert!(re.is_match(&"ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        assert!(re.is_match(&"zeDKs9LtfrB7Xm2"));
        assert!(re.is_match(&"0".repeat(256)));
    }

    #[test]
    fn regex_validate_color_code() {
        let re = RegexBuilder::new(
            r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6}|[a-f0-9]{8})\b|(?:rgb|hsl)a?\([^\)]*\)$",
        )
        .case_insensitive(true)
        .build()
        .unwrap();
        // invalids
        assert!(!re.is_match("#f2ewq"));
        assert!(!re.is_match(""));
        // valids
        assert!(re.is_match("#f2f2f2"));
        assert!(re.is_match("#F2F2F2"));
        assert!(re.is_match("#00000000"));
        assert!(re.is_match("#fff"));
        assert!(re.is_match("rgb(255,0,24)"));
        assert!(re.is_match("rgb(255, 0, 24)"));
        assert!(re.is_match("rgba(255, 0, 24, .5)"));
        assert!(re.is_match("rgba(#fff, .5)"));
        assert!(re.is_match("rgba(#fff,.5)"));
        assert!(re.is_match("rgba(#FFF, .5)"));
        assert!(re.is_match("hsl(120, 100%, 50%)"));
        assert!(re.is_match("hsl(120,100%,50%)"));
        assert!(re.is_match("hsla(170, 23%, 25%, 0.2)"));
        assert!(re.is_match("hsla(170,23%,25%,0.2)"));
        assert!(re.is_match("0x00ffff"));
        assert!(re.is_match("0x00FFFF"));
    }

    #[test]
    fn regex_validate_time() {
        let re = RegexBuilder::new(r"^(?:[01]\d|2[0-3]):[0-5]\d$")
            .build()
            .unwrap();
        // invalids
        assert!(!re.is_match("00:00:00"));
        assert!(!re.is_match("0:00"));
        assert!(!re.is_match("00:0"));
        assert!(!re.is_match("0:0"));
        assert!(!re.is_match("0:"));
        assert!(!re.is_match(":0"));
        assert!(!re.is_match(":"));
        assert!(!re.is_match("0"));
        assert!(!re.is_match(""));
        assert!(!re.is_match("24:00"));
        assert!(!re.is_match("23:60"));
        assert!(!re.is_match("-1:00"));
        assert!(!re.is_match("00:-1"));
        assert!(!re.is_match(""));
        // valids
        assert!(re.is_match("00:00"));
        assert!(re.is_match("23:59"));
    }

    #[test]
    fn regex_validate_datetime() {
        let re = RegexBuilder::new(
            r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\d|2[0-3]):[0-5]\d$"
        )
        .build()
        .unwrap();
        // invalids
        assert!(!re.is_match("0000-00-00T00:00"));
        assert!(!re.is_match("0000-00-00T00:00Z"));
        assert!(!re.is_match("0000-01-01T00:00"));
        assert!(!re.is_match("1900-01-01T00:00:00"));
        assert!(!re.is_match("1900-00-00T00:00"));
        assert!(!re.is_match("1900-13-01T00:00"));
        assert!(!re.is_match("1900-01-32T00:00"));
        assert!(!re.is_match("1900-01-01T24:00"));
        assert!(!re.is_match("1900-01-01T00:60"));
        assert!(!re.is_match("197-01-01T00:00"));
        assert!(!re.is_match("1900-1-01T00:00"));
        assert!(!re.is_match("1900-01-1T00:00"));
        assert!(!re.is_match("1900-01-01T0:00"));
        assert!(!re.is_match("1900-01-01T00:0"));
        assert!(!re.is_match("19000-01-01T00:00"));
        assert!(!re.is_match("1900-010-01T00:00"));
        assert!(!re.is_match("1900-01-010T00:00"));
        assert!(!re.is_match("1900-01-01T000:00"));
        assert!(!re.is_match("1900-01-01T00:000"));
        assert!(!re.is_match("1900/01/01T00:00"));
        assert!(!re.is_match("1900.01.01T00:00"));
        assert!(!re.is_match("1900-01-01 00:00"));
        assert!(!re.is_match("01011900"));
        assert!(!re.is_match("01/01/1900"));
        assert!(!re.is_match("01.01.1900"));
        assert!(!re.is_match("1900-01-01"));
        assert!(!re.is_match("1900-01-01 00:00"));
        assert!(!re.is_match("1900-01-01T00:00Z"));
        assert!(!re.is_match("1901-02-29T00:00"));
        assert!(!re.is_match("1995-02-29T00:00"));
        assert!(!re.is_match("1975-02-29T00:00"));
        assert!(!re.is_match("1951-02-29T00:00"));
        assert!(!re.is_match("1949-02-29T00:00"));
        assert!(!re.is_match("1942-02-29T00:00"));
        assert!(!re.is_match("1923-02-29T00:00"));
        assert!(!re.is_match("1921-02-29T00:00"));
        assert!(!re.is_match("1917-02-29T00:00"));
        assert!(!re.is_match("1913-02-29T00:00"));
        assert!(!re.is_match("1909-02-29T00:00"));
        assert!(!re.is_match("2002-02-29T00:00"));
        assert!(!re.is_match("2005-02-29T00:00"));
        assert!(!re.is_match("2009-02-29T00:00"));
        assert!(!re.is_match("2010-02-29T00:00"));
        assert!(!re.is_match("2011-02-29T00:00"));
        assert!(!re.is_match("2019-02-29T00:00"));
        assert!(!re.is_match("2023-02-29T00:00"));
        assert!(!re.is_match("1900-04-31T00:00"));
        assert!(!re.is_match("1900-06-31T00:00"));
        assert!(!re.is_match("1900-09-31T00:00"));
        assert!(!re.is_match("1900-11-31T00:00"));
        assert!(!re.is_match(""));
        // valids
        assert!(re.is_match("1900-01-31T00:00"));
        assert!(re.is_match("1904-02-29T00:00"));
        assert!(re.is_match("1996-02-29T00:00"));
        assert!(re.is_match("1972-02-29T00:00"));
        assert!(re.is_match("1952-02-29T00:00"));
        assert!(re.is_match("1948-02-29T00:00"));
        assert!(re.is_match("1940-02-29T00:00"));
        assert!(re.is_match("1924-02-29T00:00"));
        assert!(re.is_match("1920-02-29T00:00"));
        assert!(re.is_match("1916-02-29T00:00"));
        assert!(re.is_match("1912-02-29T00:00"));
        assert!(re.is_match("1908-02-29T00:00"));
        assert!(re.is_match("2000-02-29T00:00"));
        assert!(re.is_match("2004-02-29T00:00"));
        assert!(re.is_match("2008-02-29T00:00"));
        assert!(re.is_match("2012-02-29T00:00"));
        assert!(re.is_match("2016-02-29T00:00"));
        assert!(re.is_match("2020-02-29T00:00"));
        assert!(re.is_match("2024-02-29T00:00"));
        assert!(re.is_match("1900-03-31T00:00"));
        assert!(re.is_match("1900-04-30T00:00"));
        assert!(re.is_match("1900-05-31T00:00"));
        assert!(re.is_match("1900-06-30T00:00"));
        assert!(re.is_match("1900-07-31T00:00"));
        assert!(re.is_match("1900-08-31T00:00"));
        assert!(re.is_match("1900-09-30T00:00"));
        assert!(re.is_match("1900-10-31T00:00"));
        assert!(re.is_match("1900-11-30T00:00"));
        assert!(re.is_match("1900-12-31T00:00"));
        assert!(re.is_match("1000-01-01T00:00"));
        assert!(re.is_match("1900-01-01T00:00"));
        assert!(re.is_match("9999-12-31T23:59"));
        assert!(re.is_match("2020-10-16T12:52"));
    }

    #[test]
    fn regex_validate_date() {
        let re = RegexBuilder::new(
            r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)$"
        )
            .build()
            .unwrap();
        // invalids
        assert!(!re.is_match("0000-00-00"));
        assert!(!re.is_match("1900-00-00"));
        assert!(!re.is_match("1900-13-01"));
        assert!(!re.is_match("1900-01-32"));
        assert!(!re.is_match("197-01-01"));
        assert!(!re.is_match("1900-1-01"));
        assert!(!re.is_match("1900-01-1"));
        assert!(!re.is_match("19000-01-01"));
        assert!(!re.is_match("1900-010-01"));
        assert!(!re.is_match("1900-01-010"));
        assert!(!re.is_match("1900/01/01"));
        assert!(!re.is_match("1900.01.01"));
        assert!(!re.is_match("01011900"));
        assert!(!re.is_match("01/01/1900"));
        assert!(!re.is_match("01.01.1900"));
        assert!(!re.is_match("1900-01-01 00:00"));
        assert!(!re.is_match("1900-01-01T00:00"));
        assert!(!re.is_match("1900-01-01 00:00"));
        assert!(!re.is_match("1900-01-01T00:00"));
        assert!(!re.is_match("1900-01-01T00:00:00Z"));
        assert!(!re.is_match("9999-12-31T23:59:59"));
        assert!(!re.is_match("1900-01-01T00:00"));
        assert!(!re.is_match("1901-02-29"));
        assert!(!re.is_match("2002-02-29"));
        assert!(!re.is_match("2005-02-29"));
        assert!(!re.is_match("2009-02-29"));
        assert!(!re.is_match("2010-02-29"));
        assert!(!re.is_match("2011-02-29"));
        assert!(!re.is_match("2019-02-29"));
        assert!(!re.is_match("2023-02-29"));
        assert!(!re.is_match("1995-02-29"));
        assert!(!re.is_match("1975-02-29"));
        assert!(!re.is_match("1951-02-29"));
        assert!(!re.is_match("1949-02-29"));
        assert!(!re.is_match("1942-02-29"));
        assert!(!re.is_match("1923-02-29"));
        assert!(!re.is_match("1921-02-29"));
        assert!(!re.is_match("1917-02-29"));
        assert!(!re.is_match("1913-02-29"));
        assert!(!re.is_match("1909-02-29"));
        assert!(!re.is_match("1900-04-31"));
        assert!(!re.is_match("1900-06-31"));
        assert!(!re.is_match("1900-09-31"));
        assert!(!re.is_match("1900-11-31"));
        assert!(!re.is_match(""));
        // valids
        assert!(re.is_match("1900-01-31"));
        assert!(re.is_match("1904-02-29"));
        assert!(re.is_match("1996-02-29"));
        assert!(re.is_match("1972-02-29"));
        assert!(re.is_match("1952-02-29"));
        assert!(re.is_match("1948-02-29"));
        assert!(re.is_match("1940-02-29"));
        assert!(re.is_match("1924-02-29"));
        assert!(re.is_match("1920-02-29"));
        assert!(re.is_match("1916-02-29"));
        assert!(re.is_match("1912-02-29"));
        assert!(re.is_match("1908-02-29"));
        assert!(re.is_match("2000-02-29"));
        assert!(re.is_match("2004-02-29"));
        assert!(re.is_match("2008-02-29"));
        assert!(re.is_match("2012-02-29"));
        assert!(re.is_match("2016-02-29"));
        assert!(re.is_match("2020-02-29"));
        assert!(re.is_match("2024-02-29"));
        assert!(re.is_match("1900-03-31"));
        assert!(re.is_match("1900-04-30"));
        assert!(re.is_match("1900-05-31"));
        assert!(re.is_match("1900-06-30"));
        assert!(re.is_match("1900-07-31"));
        assert!(re.is_match("1900-08-31"));
        assert!(re.is_match("1900-09-30"));
        assert!(re.is_match("1900-10-31"));
        assert!(re.is_match("1900-11-30"));
        assert!(re.is_match("1900-12-31"));
        assert!(re.is_match("1000-01-01"));
        assert!(re.is_match("1900-01-01"));
        assert!(re.is_match("9999-12-31"));
        assert!(re.is_match("2020-10-15"));
    }

    // Regular expressions
    // *********************************************************************************************
    #[test]
    fn regex_validate_json_dyn_widgets() {
        let re = RegexBuilder::new(r#"^\{[\s]*(?:"[a-z][a-z\d]*(?:_[a-z\d]+)*":(?:\[(?:(?:\["[-_.\s\w]+","[-_.\s\w]+"\])(?:,\["[-_.\s\w]+","[-_.\s\w]+"\])*)*\]))(?:,[\s]*"[a-z][a-z\d]*(?:_[a-z\d]+)*":(?:\[(?:(?:\["[-_.\s\w]+","[-_.\s\w]+"\])(?:,\["[-_.\s\w]+","[-_.\s\w]+"\])*)*\]))*[\s]*\}$"#)
            .case_insensitive(true)
            .build()
            .unwrap();
        // invalids
        assert!(!re.is_match(r#""#));
        assert!(!re.is_match(r#"{}"#));
        assert!(!re.is_match(r#"{""}"#));
        assert!(!re.is_match(r#"{:}"#));
        assert!(!re.is_match(r#"{[]}"#));
        assert!(!re.is_match(r#"{[[]]}"#));
        assert!(!re.is_match(r#"{":}"#));
        assert!(!re.is_match(r#"{"":}"#));
        assert!(!re.is_match(r#"{:[}"#));
        assert!(!re.is_match(r#"{:]}"#));
        assert!(!re.is_match(r#"{:[[}"#));
        assert!(!re.is_match(r#"{"":[]}"#));
        assert!(!re.is_match(r#"{"":[[]]}"#));
        assert!(!re.is_match(r#"{"2":[[]]}"#));
        assert!(!re.is_match(r#"{"2abc":[[]]}"#));
        assert!(!re.is_match(r#"{"2abc_def":[[]]}"#));
        assert!(!re.is_match(r#"{"_abc":[[]]}"#));
        assert!(!re.is_match(r#"{"2abc":[[]]}"#));
        assert!(!re.is_match(r#"{"abc def":[[]]}"#));
        assert!(!re.is_match(r#"{"abc_def"}"#));
        assert!(!re.is_match(r#"{"abc_def":}"#));
        assert!(!re.is_match(r#"{"abc_def":[}"#));
        assert!(!re.is_match(r#"{"abc_def":[[}"#));
        assert!(!re.is_match(r#"{"abc_def":]}"#));
        assert!(!re.is_match(r#"{"abc_def":]]}"#));
        assert!(!re.is_match(r#"{"abc_def":["",""]}"#));
        assert!(!re.is_match(r#"{"abc_def":["value"]}"#));
        assert!(!re.is_match(r#"{"abc_def":["value",]}"#));
        assert!(!re.is_match(r#"{"abc_def":["value",""]}"#));
        assert!(!re.is_match(r#"{"abc_def":["value","Title"]}"#));
        assert!(!re.is_match(r#"{"abc_def":["value","Title",]}"#));
        assert!(!re.is_match(r#"{"abc_def":["value","Title"],}"#));
        assert!(!re.is_match(r#"{"abc_def":["value","Title",],}"#));
        assert!(!re.is_match(r#"{"abc_def":[["value","Title"]}"#));
        assert!(!re.is_match(r#"{"abc_def":["value","Title"]]}"#));
        assert!(!re.is_match(r#"{"abc_def":[["value","Title"],]}"#));
        assert!(!re.is_match(r#"{"abc_def":[["value","Title"]],}"#));
        assert!(!re.is_match(r#"{"abc_def":[["value","Title"]],,}"#));
        assert!(!re.is_match(r#"{"abc_def":[["value" "Title"]]}"#));
        assert!(!re.is_match(r#"{"abc_def":[["value""Title"]]}"#));
        assert!(!re.is_match(r#"{"abc_def":[["value","Title"],],}"#));
        assert!(
            !re.is_match(r#"{"field_name":[["value","Title"]]"field_name":[["value","Title"]]}"#)
        );
        assert!(
            !re.is_match(r#"{"field_name":[["value","Title"]]"field_name":[["value","Title"]]"field_name":[["value","Title"]]}"#)
        );
        assert!(
            !re.is_match(r#"{"field_name":[["value","Title"]],"field_name":[["value","Title"]]"field_name":[["value","Title"]]}"#)
        );
        assert!(
            !re.is_match(r#"{"field_name":[["value","Title"]]"field_name":[["value","Title"]],"field_name":[["value","Title"]]}"#)
        );
        assert!(
            !re.is_match(r#"{"field_name":["value"],"field_name":[["value","Title"]],"field_name":[["value","Title"]]}"#)
        );
        assert!(
            !re.is_match(r#"{"field_name":[["value","Title"]],"field_name":["Title"],"field_name":[["value","Title"]]}"#)
        );
        assert!(
            !re.is_match(r#"{"field_name":[["value","Title"]],"field_name":[["value","Title"]],"field_name":["value"]}"#)
        );
        assert!(!re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]]
                "field_name":[["value","Title"],["value","Title"]]
                "field_name":[["value","Title"],["value","Title"]]
            }"#
        ));
        assert!(!re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]]
                "field_name":[["value","Title"],["value","Title"]]
            }"#
        ));
        assert!(!re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]]
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]]
            }"#
        ));
        assert!(!re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]]
                "field_name":[["value","Title"],["value","Title"]]
                "field_name":[["value","Title"],["value","Title"]],
            }"#
        ));
        assert!(!re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]]
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]],
            }"#
        ));
        assert!(!re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]]
                "field_name":[["value","Title"],["value","Title"]],
            }"#
        ));
        assert!(!re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]],
            }"#
        ));
        // valids
        assert!(re.is_match(r#"{"field":[]}"#));
        assert!(re.is_match(r#"{"field_name":[]}"#));
        assert!(re.is_match(r#"{"field_name_2":[]}"#));
        assert!(re.is_match(r#"{"field":[["value","Title"]]}"#));
        assert!(re.is_match(r#"{"field":[["2","Title"]]}"#));
        assert!(re.is_match(r#"{"field":[["-2","Title"]]}"#));
        assert!(re.is_match(r#"{"field":[["2.2","Title"]]}"#));
        assert!(re.is_match(r#"{"field":[["-2.2","Title"]]}"#));
        assert!(re.is_match(r#"{"field":[["2","Title 2"]]}"#));
        assert!(re.is_match(r#"{"field":[["-2","Title 2"]]}"#));
        assert!(re.is_match(r#"{"field":[["2.2","Title 2"]]}"#));
        assert!(re.is_match(r#"{"field":[["-2.2","Title 2"]]}"#));
        assert!(re.is_match(r#"{"field_name":[["value","Title"]]}"#));
        assert!(re.is_match(r#"{"field_name2":[["value","Title"]]}"#));
        assert!(re.is_match(r#"{"field_name_2":[["value","Title"]]}"#));
        assert!(re.is_match(r#"{"field_name":[["value","Title"],["value","Title"]]}"#));
        assert!(re
            .is_match(r#"{"field_name":[["value","Title"],["value","Title"],["value","Title"]]}"#));
        assert!(re.is_match(r#"{"field_name":[],"field_name_2":[]}"#));
        assert!(re.is_match(r#"{"field_name":[["value","Title"]],"field_name_2":[]}"#));
        assert!(re.is_match(r#"{"field_name":[],"field_name_2":[["value","Title"]]}"#));
        assert!(
            re.is_match(r#"{"field_name":[["value","Title"]],"field_name_2":[["value","Title"]]}"#)
        );
        assert!(
            re.is_match(r#"{"field_name":[["value","Title"]],"field_name":[["value","Title"]],"field_name":[["value","Title"]]}"#)
        );
        assert!(
            re.is_match(r#"{"field_name":[["value","Title"],["value","Title"]],"field_name":[["value","Title"]]}"#)
        );
        assert!(
            re.is_match(r#"{"field_name":[["value","Title"]],"field_name":[["value","Title"],["value","Title"]]}"#)
        );
        assert!(
            re.is_match(r#"{"field_name":[["value","Title"],["value","Title"]],"field_name":[["value","Title"],["value","Title"]]}"#)
        );
        assert!(
            re.is_match(r#"{"field_name":[["value","Title"],["value","Title"]],"field_name":[["value","Title"],["value","Title"]],"field_name":[["value","Title"],["value","Title"]]}"#)
        );
        assert!(re.is_match(
            r#"{
                "field_name":[],
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]]
            }"#
        ));
        assert!(re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[],
                "field_name":[["value","Title"],["value","Title"]]
            }"#
        ));
        assert!(re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[]
            }"#
        ));
        assert!(re.is_match(
            r#"{
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]],
                "field_name":[["value","Title"],["value","Title"]]
            }"#
        ));
        assert!(re.is_match(
            r#"{
                "field_name":[["value","Title"],["value2","Title 2"]],
                "field_name":[["value","Title"],["value2","Title 2"]],
                "field_name":[["value","Title"],["value2","Title 2"]]
            }"#
        ));
        assert!(re.is_match(
            r#"{
                "select_text_dyn":[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]],
                "select_text_mult_dyn":[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]],
                "select_i32_dyn":[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]],
                "select_i32_mult_dyn":[["-1","Volvo"],["-2","Saab"],["-3","Mercedes"],["-4","Audi"]],
                "select_u32_dyn":[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]],
                "select_u32_mult_dyn":[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]],
                "select_i64_dyn":[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]],
                "select_i64_mult_dyn":[["-1","Volvo"],["-2","Saab"],["-3","Mercedes"],["-4","Audi"]],
                "select_f64_dyn":[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]],
                "select_f64_mult_dyn":[["-1.1","Volvo"],["-2.2","Saab"],["-3.3","Mercedes"],["-4.4","Audi"]]
            }"#
        ));
        assert!(re.is_match(
            r#"{
                "select_text_dyn":[],
                "select_text_mult_dyn":[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]],
                "select_i32_dyn":[],
                "select_i32_mult_dyn":[["-1","Volvo"],["-2","Saab"],["-3","Mercedes"],["-4","Audi"]],
                "select_u32_dyn":[],
                "select_u32_mult_dyn":[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]],
                "select_i64_dyn":[],
                "select_i64_mult_dyn":[["-1","Volvo"],["-2","Saab"],["-3","Mercedes"],["-4","Audi"]],
                "select_f64_dyn":[],
                "select_f64_mult_dyn":[["-1.1","Volvo"],["-2.2","Saab"],["-3.3","Mercedes"],["-4.4","Audi"]]
            }"#
        ));
        assert!(re.is_match(
            r#"{
                "select_text_dyn":[],
                "select_text_mult_dyn":[],
                "select_i32_dyn":[],
                "select_i32_mult_dyn":[],
                "select_u32_dyn":[],
                "select_u32_mult_dyn":[],
                "select_i64_dyn":[],
                "select_i64_mult_dyn":[],
                "select_f64_dyn":[],
                "select_f64_mult_dyn":[]
            }"#
        ));
    }
}
