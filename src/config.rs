use std::fs;
use std::io;

pub fn read_source(args: &[String]) -> Result<String, io::Error> {
    let source_file = args[1].clone();
    let source_code = fs::read_to_string(source_file)?;
    Ok(source_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_source_valid() {
        let args = vec![String::new(), String::from("test_source/addition.c")];
        let result = read_source(&args);
        assert!(!result.is_err());
    }
    #[test]
    fn test_read_source_invalid() {
        let args = vec![String::new(), String::from("invaliddirectory/invalidsourcefile.c")];
        let result = read_source(&args);
        assert!(result.is_err());
    }
}
