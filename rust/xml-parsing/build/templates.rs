pub const FIELD_STRUCT: &'static str = r#"
use std::fmt::Display;

{{#each fields}}
#[derive(Debug)]
pub struct {{this.name}}Field {
    tag: u32,
    value: String,
} 

impl {{this.name}}Field {
    fn new<T: Into<{{fld_type}}> + Display>(val: T) -> Self {
        Self {
            tag: {{number}},
            value: val.to_string()
        }
    }
}
{{/each}}

"#;

// {{#if values}}
// #[allow(non_camel_case_types)]
// #[derive(Debug)]
// pub enum {{name}} {
//     {{#each values}}
//     {{this.enum_variant}},
//     {{/each}}
// }

// impl {{name}} {
//     fn get(&self) -> String {
//         match self {
//             {{#each values}}
//             {{this.enum_variant}} => String::from("{{this.variant_val}}"),
//             {{/each}}
//         }
//     }

// }
// {{/if}}
// "#;

// const FIELD_ENUM: &'static str = r#"
// #[derive(Debug)]
// enum {{name}} {
//     {{#each values}}
//     {{this.enum_variant}},
//     {{/each}}
// }
// "#;
