pub mod checksum;
pub mod parse;

use crate::traits::CommandArg;

pub fn commands() -> Vec<Box<dyn CommandArg>> {
    vec![
        Box::new(checksum::ChecksumCommand::new()),
        Box::new(parse::ParseCommand::new()),
    ]
}
