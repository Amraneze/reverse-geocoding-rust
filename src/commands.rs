use crate::types::Command;
use std::fmt::Debug;
use std::str::FromStr;
use std::string::ToString;

const LOOKUP: &'static str = "LOOKUP";

#[derive(Debug)]
pub enum Commands {
    Lookup(f32, f32),
}

#[derive(Debug)]
pub enum CommandsError {
    NoCommandProvided(),
    UnknownCommand(String),
    InvalidNumberOfArgs(String, usize, usize),
}

impl From<CommandsError> for String {
    fn from(error: CommandsError) -> String {
        return match error {
            CommandsError::NoCommandProvided() => "No command was provided".to_string(),
            CommandsError::UnknownCommand(command) => format!("Invalid command {}", command),
            CommandsError::InvalidNumberOfArgs(command, required, given) => {
                format!(
                    "Command {} requires {} of arguments, but only {} were given",
                    command, required, given
                )
            }
        };
    }
}

pub(crate) trait FromString {
    fn from_str(string: &str) -> Self;
}

impl FromString for Command {
    fn from_str(command_values: &str) -> Command {
        let binding = command_values
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<String>>();
        let commands_slices: &[String] = binding.as_slice();
        match commands_slices {
            [] => Err(CommandsError::NoCommandProvided()),
            [command, longitude, latitude] => {
                if command.starts_with(LOOKUP) {
                    return Ok(Commands::Lookup(
                        f32::from_str(&longitude).unwrap(),
                        f32::from_str(&latitude).unwrap(),
                    ));
                }
                return Err(CommandsError::UnknownCommand((&command).to_string()));
            }
            _ => Err(CommandsError::InvalidNumberOfArgs(
                (&commands_slices.first().unwrap()).to_string(),
                3,
                commands_slices.len() - 1,
            )),
        }
    }
}
