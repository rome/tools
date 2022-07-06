mod configuration {
    use rome_service::configuration::Configuration;
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
}
