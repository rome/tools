pub const CONFIG_FORMAT: &str = r#"{
  "root": true,
  "formatter": {
    "enabled": true,
    "lineWidth": 160,
    "indentStyle": "space",
    "indentSize": 6
  }
}
"#;

pub const CONFIG_DISABLED_FORMATTER: &str = r#"{
  "root": true,
  "formatter": {
    "enabled": false
  }
}
"#;

pub const CONFIG_ROOT_FALSE: &str = r#"{
    "root": false
}
"#;

pub const CONFIG_ALL_FIELDS: &str = r#"{
  "root": true,
  "formatter": {
    "enabled": true,
    "formatWithErrors": true,
    "indentStyle": "tab",
    "indentSize": 2,
    "lineWidth": 80
  },
  "javascript": {
    "formatter": {
      "quoteStyle": "double"
    }
  }
}"#;

pub const CONFIG_BAD_LINE_WIDTH: &str = r#"{
  "root": true,
  "formatter": {
    "lineWidth": 500
  },
}"#;
