extern crate core;

mod configuration {
    use rome_service::configuration::Configuration;
    use rome_service::load_config;
    use rome_service::load_config::ConfigurationType;
    use std::env::current_dir;
    use std::fs::read_to_string;

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
        let mut working_dir = current_dir().unwrap();
        working_dir.push("tests");
        working_dir.push("empty.json");

        let result = load_config(&working_dir, ConfigurationType::Root);

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
        let mut working_dir = current_dir().unwrap();
        working_dir.push("tests");
        working_dir.push("all_fields.json");

        let result = load_config(&working_dir, ConfigurationType::Root);

        assert!(result.is_ok());
    }
}
