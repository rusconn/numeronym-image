use std::env;

pub struct Config {
    pub word: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let word = match args.next() {
            Some(arg) => arg,
            None => return Err("pass a word using command line argument."),
        };

        Ok(Config { word })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let numero = numeronym(&config.word)?;

    println!("{}", numero);

    Ok(())
}

fn numeronym(word: &str) -> Result<String, &'static str> {
    if word.len() < 3 {
        return Err("word must be at least 3 in length.");
    }

    let mut chars = word.chars();
    let first = chars.next().unwrap();
    let numero = word.len() - 2;
    let last = chars.last().unwrap();

    return Ok(format!("{}{}{}", first, numero, last));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let word = "kubernetes";
        assert_eq!(numeronym(word).unwrap(), "k8s".to_string());
    }

    #[test]
    fn test_2() {
        let word = "internationalization";
        assert_eq!(numeronym(word).unwrap(), "i18n".to_string());
    }

    #[test]
    fn test_3() {
        let word = "hi";
        assert!(numeronym(word).is_err());
    }
}
