pub const CONFIG_FORMAT: &str = r#"{
  "formatter": {
    "enabled": true,
    "lineWidth": 160,
    "indentStyle": "space",
    "indentSize": 6
  }
}
"#;

pub const CONFIG_INIT_DEFAULT: &str = r#"{
  "linter": {
    "enabled": true,
    "rules": {
      "recommended": true
    }
  }
}"#;

pub const CONFIG_DISABLED_FORMATTER: &str = r#"{
  "formatter": {
    "enabled": false
  }
}
"#;

pub const CONFIG_ALL_FIELDS: &str = r#"{
  "formatter": {
    "enabled": true,
    "formatWithErrors": true,
    "indentStyle": "tab",
    "indentSize": 2,
    "lineWidth": 80
  },
  "linter": {
    "enabled": true,
    "rules": {
        "js": {
            "noDeadCode": "off",
            "useSimplifiedLogicExpression": "warn",
            "noCatchAssign": "error",
            "noLabelVar": {
                "level": "warn"
            },
            "useTemplate": {
                "level": "error"
            }
        }
    }
  },
  "javascript": {
    "globals": ["$"],
    "formatter": {
      "quoteStyle": "double",
      "quoteProperties": "asNeeded"
    }
  }
}"#;

pub const CONFIG_BAD_LINE_WIDTH: &str = r#"{
  "formatter": {
    "lineWidth": 500
  }
}"#;

pub const CONFIG_LINTER_DISABLED: &str = r#"{
  "linter": {
    "enabled": false
  }
}"#;

pub const CONFIG_LINTER_WRONG_RULE: &str = r#"{
  "linter": {
    "enabled": true,
    "rules": {
        "js": {
            "foo_rule": "off"
        },
        "jsx": {
            "what_the_hell": "off"
        }
    }
  }
}"#;

pub const CONFIG_INCORRECT_GLOBALS: &str = r#"{
  "linter": {
    "enabled": false
  },
  "javascript": {
    "globals": [false]
  }
}"#;

pub const CONFIG_LINTER_SUPPRESSED_RULE: &str = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "js": {
            "noDebugger": "off"
        }
    }
  }
}"#;

pub const CONFIG_LINTER_SUPPRESSED_GROUP: &str = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "js": {
            "recommended": false
        }
    }
  }
}"#;

pub const CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC: &str = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "js": {
            "recommended": true,      
            "noDebugger": "warn"
        }
    }
  }
}"#;

pub const CONFIG_LINTER_UPGRADE_DIAGNOSTIC: &str = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "js": {
            "noDeadCode": "error"
        }
    }
  }
}"#;

pub const CONFIG_INCORRECT_GLOBALS_V2: &str = r#"{
    "javascript": {
      "formatter": {
        "quoteStyle": "single"
      }
  }
}"#;

pub const CONFIG_ISSUE_3175_1: &str = r#"{
  "formatter": {
    "indentStyle": "space",
    "indentSize": 2,
    "lineWidth": 120
  }
}"#;

pub const CONFIG_ISSUE_3175_2: &str = r#"{
  "javascript": {
    "formatter": {
        "quoteStyle": "single"
    }
  }
}"#;
