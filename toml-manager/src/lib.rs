use std::collections::HashMap;
use toml::{Table, Value};
use value::XmgrValue;

pub mod package;
pub mod value;

#[derive(Debug)]
pub struct XmgrConfig {
    pub machines: HashMap<String, XmgrMachine>,
    pub global: HashMap<String, XmgrGlobal>,
}

#[derive(Debug)]
pub struct XmgrMachine {
    pub host_platform: String,
    pub hostname: String,
    pub packages: Vec<package::XmgrPackage>,
}

pub struct XmgrUser {
    pub name: String,
}

#[derive(Debug)]
pub struct XmgrGlobal {
    pub packages: Vec<package::XmgrPackage>,
}

#[derive(Debug)]
pub enum TomlError {
    ParseError(toml::de::Error),
    GlobalMissing,
    GlobalInvalid,
}

pub fn parse(input: &str) -> Result<XmgrConfig, TomlError> {
    let parsed = toml::from_str::<Table>(input).map_err(TomlError::ParseError)?;
    let mut result = XmgrConfig {
        machines: HashMap::new(),
        global: HashMap::new(),
    };

    let global = parsed
        .get("global")
        .ok_or(TomlError::GlobalMissing)
        .map(|value| match value {
            Value::Table(value) => Ok(value),
            _ => Err(TomlError::GlobalInvalid),
        })?;

    result.global.extend(
        global
            .map_err(|_| TomlError::GlobalInvalid)?
            .into_iter()
            .map(|(key, value)| {
                println!("{:?}", key);
                println!("{:?}", value);
                (
                    key.clone(),
                    XmgrGlobal {
                        packages: match value {},
                    },
                )
            })
            .collect(),
    );
    println!("-----");

    println!("{:?}", parsed);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), std::io::Error> {
        let content = std::fs::read_to_string("./artifacts/xinux-manager.toml")?;
        let result = parse(&content);
        println!("{:?}", result);
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        Ok(())
    }
}
