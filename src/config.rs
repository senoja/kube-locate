#[derive(Debug,PartialEq)]
pub enum Mode {
    Normal,
    Interactive,
}

#[derive(Debug)]
pub struct Opt {
    pub arg1: Option<String>,
    pub arg2: Option<String>,
    pub mode: Mode,
}

impl Opt {
    pub fn new_from_args(mut args: impl Iterator<Item = String>) -> Result<Opt, &'static str> { // TODO: refactor errors to remove str types
        args.next(); // skip the executable path

        let arg1 = match args.next() { // first arg: context name
            Some(arg) => {
                if arg == "--help" || arg == "-h" {
                    return Err("invalid context name")
                }
                Some(arg)
            },
            None => return Ok(Opt{
                arg1: None,
                arg2: None,
                mode: Mode::Interactive,
            }),
        };

        let arg2 = match args.next() { // second arg: optional namespace name
            Some(arg) => Some(arg),
            None => None,
        };

        if args.next().is_some() {
            return Err("received too many arguments")
        };

        Ok(Opt{
            arg1,
            arg2,
            mode: Mode::Normal,
        })

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_empty_args() {
        let args = vec!["binary".to_string()].into_iter();

        let config = match Opt::new_from_args(args) {
            Ok(c) => c,
            Err(e) => panic!("should not throw an error when called with no arguments: {}", e),
        };

        assert_eq!(config.mode, Mode::Interactive, "mode should be interactive if no arguments are provided");
    }

    #[test]
    fn config_help_flag() {
        let args = vec!["binary".to_string(), "-h".to_string()].into_iter();
        assert!(Opt::new_from_args(args).is_err(), "-h passed as the first argument should throw an error");

        let args = vec!["binary".to_string(), "--help".to_string()].into_iter();
        assert!(Opt::new_from_args(args).is_err(), "--help passed as the first argument should throw an error");
    }

    #[test]
    fn config_new_args() {
        let args = vec!["binary".to_string(),
                        "context".into(), "namespace".into()].into_iter();

        let config = match Opt::new_from_args(args) {
            Ok(c) => c,
            Err(e) => panic!("should not throw an error with valid args: {}", e),
        };

        assert_eq!(config.arg1, Some("context".to_string()), "context field should be set from arg 1");
        assert_eq!(config.arg2, Some("namespace".to_string()), "namespace field should be set from arg 2");
        assert_eq!(config.mode, Mode::Normal, "mode should be normal if arguments are provided");
    }

    #[test]
    fn new_config_extra_args() {
        let args = vec!["binary".to_string(),
                        "arg1".into(), "namespace".into(),
                        "amistake".into()].into_iter();

        if let Ok(_) = Opt::new_from_args(args) {
            panic!("config should return an error if there are more than 2 args");
        }
    }
}
