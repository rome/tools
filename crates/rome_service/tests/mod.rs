extern crate core;

const CONFIG_EMPTY: &str = r#"{}
"#;

const CONFIG_ALL_FIELDS: &str = r#"{
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

mod configuration {
    use crate::{CONFIG_ALL_FIELDS, CONFIG_EMPTY};
    use rome_console::BufferConsole;
    use rome_fs::MemoryFileSystem;
    use rome_service::configuration::Configuration;
    use rome_service::load_config::ConfigurationType;
    use rome_service::{load_config, App, DynRef};
    use std::env::current_dir;
    use std::fs::read_to_string;
    use std::path::Path;

    #[test]
    fn parse_all_fields() {
        let mut working_dir = current_dir().unwrap();
        working_dir.push("tests");
        working_dir.push("all_fields.json");
        let content = read_to_string(working_dir).unwrap();

        let configuration = serde_json::from_str::<Configuration>(&content);

        assert!(configuration.is_ok())
    }

    #[test]
    fn parse_default_values() {
        let mut working_dir = current_dir().unwrap();
        working_dir.push("tests");
        working_dir.push("empty.json");
        let content = read_to_string(working_dir).unwrap();

        let configuration = serde_json::from_str::<Configuration>(&content);

        dbg!(Configuration::default());
        match configuration {
            Ok(configuration) => {
                assert_eq!(configuration, Configuration::default());
            }
            Err(err) => {
                panic!("{err}");
            }
        }
    }

    #[test]
    fn line_width_error() {
        let mut working_dir = current_dir().unwrap();
        working_dir.push("tests");
        working_dir.push("line_width.json");
        let content = read_to_string(working_dir).unwrap();

        let configuration = serde_json::from_str::<Configuration>(&content);

        assert!(configuration.is_err());

        if let Err(err) = configuration {
            assert!(err
                .to_string()
                .as_str()
                .contains("The line width exceeds the maximum value (320)"),)
        }
    }

    #[test]
    fn incorrect_root() {
        let mut fs = MemoryFileSystem::default();
        let config_path = current_dir().unwrap().join("rome.json");
        let file_path = Path::new(config_path.as_os_str());
        fs.insert(file_path.into(), CONFIG_EMPTY.as_bytes());

        let app = App::with_filesystem_and_console(
            DynRef::Borrowed(&mut fs),
            DynRef::Owned(Box::new(BufferConsole::default())),
        );

        let result = load_config(&app.fs, ConfigurationType::Root);

        assert!(result.is_err());

        match result {
            Err(error) => {
                assert_eq!(
                    error.to_string(),
                    "the main configuration file, rome.json, must have the field 'root' set to `true`"
                )
            }
            _ => panic!("expected an error, but found none"),
        }
    }

    #[test]
    fn correct_root() {
        let mut fs = MemoryFileSystem::default();
        let config_path = current_dir().unwrap().join("rome.json");
        let file_path = Path::new(config_path.as_os_str());
        fs.insert(file_path.into(), CONFIG_ALL_FIELDS.as_bytes());

        let app = App::with_filesystem_and_console(
            DynRef::Borrowed(&mut fs),
            DynRef::Owned(Box::new(BufferConsole::default())),
        );

        let result = load_config(&app.fs, ConfigurationType::Root);

        assert!(result.is_ok());
    }
}
