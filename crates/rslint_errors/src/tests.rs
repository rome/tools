use termcolor::WriteColor;

use crate::{file::SimpleFile, Diagnostic, Emitter};

pub struct TestEmitter {
    str: String,
}

impl std::io::Write for TestEmitter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf = std::str::from_utf8(buf).unwrap();
        print!("{}", buf);
        self.str.push_str(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let _ = std::io::stdout().flush();
        Ok(())
    }
}

impl WriteColor for TestEmitter {
    fn supports_color(&self) -> bool {
        false
    }

    fn set_color(&mut self, _: &termcolor::ColorSpec) -> std::io::Result<()> {
        Ok(())
    }

    fn reset(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
pub fn long_line() {
    let code = "export default function some_function_name() { return 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'; }";
    let file = SimpleFile::new("file.js".to_string(), code.to_string());
    let mut emitter = Emitter::new(&file);

    let mut writer = TestEmitter { str: String::new() };

    let d =
        Diagnostic::error(0, "ERRORCODE", "Can't have function here").primary(0usize..100, "na-na");
    emitter.emit_with_writer(&d, &mut writer).unwrap();
}
