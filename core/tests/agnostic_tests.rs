use anyhow::anyhow;
use std::io::Write;
use typeshare_core::{
    context::{ParseContext, ParseFileContext},
    language::{CrateTypes, Kotlin, Language, TypeScript},
    parser::{self},
};

/// Parse and generate types for a single Rust input file.
pub fn process_input(
    input: &str,
    language: &mut dyn Language,
    imports: &CrateTypes,
    out: &mut dyn Write,
) -> anyhow::Result<()> {
    let parse_context = ParseContext::default();

    let parsed_data = parser::parse(
        &parse_context,
        ParseFileContext {
            source_code: input.to_string(),
            crate_name: "default_name".into(),
            file_name: "file_name".into(),
            file_path: "file_path".into(),
        },
    )
    .map_err(|err| anyhow!("Failed to parse {err}"))?
    .unwrap();

    if !parsed_data.errors.is_empty() {
        return Err(anyhow!("{}", parsed_data.errors[0].error));
    }

    language.generate_types(out, imports, parsed_data)?;
    Ok(())
}

mod blocklisted_types {
    use std::collections::HashMap;

    use super::*;

    fn assert_type_is_blocklisted(ty: &str, blocklisted_type: &str, column: &str) {
        let source = format!(
            r##"
    #[typeshare]
    #[serde(default, rename_all = "camelCase")]
    pub struct Foo {{
        pub bar: {ty},
    }}
    "##
        );

        let mut out: Vec<u8> = Vec::new();
        let error = process_input(
            &source,
            &mut TypeScript::default(),
            &HashMap::new(),
            &mut out,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            format!("Failed to parse a Rust type: Unsupported type: \"{blocklisted_type}\", on line 5 and column {column}")
        );
    }

    #[test]
    fn test_i64_blocklisted_struct() {
        assert_type_is_blocklisted("i64", "i64", "17");
    }

    #[test]
    fn test_u64_blocklisted_struct() {
        assert_type_is_blocklisted("u64", "u64", "17");
    }

    #[test]
    fn test_isize_blocklisted_struct() {
        assert_type_is_blocklisted("isize", "isize", "17");
    }

    #[test]
    fn test_usize_blocklisted_in_struct() {
        assert_type_is_blocklisted("usize", "usize", "17");
    }

    #[test]
    fn test_optional_blocklisted_struct() {
        assert_type_is_blocklisted("Option<i64>", "i64", "24");
    }

    #[test]
    fn test_vec_blocklisted_struct() {
        assert_type_is_blocklisted("Vec<i64>", "i64", "21");
    }

    #[test]
    fn test_hashmap_blocklisted_struct() {
        assert_type_is_blocklisted("HashMap<String, i64>", "i64", "33");
    }
}

mod serde_attributes_on_enums {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn content_not_allowed_on_non_algebraic() {
        let source = r##"
    #[typeshare]
    #[serde(content = "bla")]
    pub enum Foo {
        Variant1,
        Variant2,
    }
    "##;

        let mut out: Vec<u8> = Vec::new();
        let err = process_input(
            source,
            &mut TypeScript::default(),
            &HashMap::new(),
            &mut out,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "The serde content attribute is not supported for non-algebraic enums: Foo, on line 2 and column 4"
        );
    }

    #[test]
    fn both_not_allowed_on_non_algebraic() {
        let source = r##"
    #[typeshare]
    #[serde(tag = "bla", content = "bla2")]
    pub enum Foo {
        Variant1,
        Variant2,
    }
    "##;

        let mut out: Vec<u8> = Vec::new();
        let err = process_input(
            source,
            &mut TypeScript::default(),
            &HashMap::new(),
            &mut out,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "The serde content attribute is not supported for non-algebraic enums: Foo, on line 2 and column 4"
        );
    }

    #[test]
    fn tag_required_with_content_on_algebraic() {
        let source = r##"
    #[typeshare]
    #[serde(content = "bla")]
    pub enum Foo {
        Variant1(String),
        Variant2,
    }
    "##;

        let mut out: Vec<u8> = Vec::new();
        let err = process_input(
            source,
            &mut TypeScript::default(),
            &HashMap::new(),
            &mut out,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "Serde tag attribute needs to be specified for algebraic enum Foo. e.g. #[serde(tag = \"type\")] or #[serde(tag = \"type\", content = \"content\")], on line 2 and column 4"
        );
    }

    #[test]
    fn internally_tagged_primitive_variant_not_allowed() {
        let source = r##"
    #[typeshare]
    #[serde(tag = "type")]
    pub enum Foo {
        Variant1 { field: String },
        Variant2(String),
    }
    "##;

        let mut out: Vec<u8> = Vec::new();
        let err = process_input(
            source,
            &mut TypeScript::default(),
            &HashMap::new(),
            &mut out,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "Tuple variant Variant2 of internally tagged enum Foo must contain a struct or a map, because serde writes the tag into the same object as the variant's fields, on line 6 and column 8"
        );
    }

    #[test]
    fn internally_tagged_optional_variant_not_allowed() {
        let source = r##"
    #[typeshare]
    #[serde(tag = "type")]
    pub enum Foo {
        Variant1(Option<Bar>),
    }
    "##;

        let mut out: Vec<u8> = Vec::new();
        let err = process_input(
            source,
            &mut TypeScript::default(),
            &HashMap::new(),
            &mut out,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "Tuple variant Variant1 of internally tagged enum Foo must contain a struct or a map, because serde writes the tag into the same object as the variant's fields, on line 5 and column 8"
        );
    }

    #[test]
    fn internally_tagged_not_supported_for_kotlin() {
        let source = r##"
    #[typeshare]
    #[serde(tag = "type")]
    pub enum Foo {
        Variant1 { field: String },
    }
    "##;

        let mut out: Vec<u8> = Vec::new();
        let err =
            process_input(source, &mut Kotlin::default(), &HashMap::new(), &mut out).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Internally tagged enums are not supported for Kotlin: Foo. Add a content key, e.g. #[serde(tag = \"type\", content = \"content\")]"
        );
    }

    #[test]
    fn no_flatten() {
        let source = r##"
        #[typeshare]
        pub struct Foo {
            #[serde(flatten)]
            pub field1: HashMap<String, String>
        }
        "##;

        let mut out: Vec<u8> = Vec::new();
        let err = process_input(
            source,
            &mut TypeScript::default(),
            &HashMap::new(),
            &mut out,
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "The serde flatten attribute is not currently supported, on line 4 and column 12"
        );
    }
}
