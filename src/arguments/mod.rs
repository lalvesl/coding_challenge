pub mod checksum;
pub mod parse;

use crate::traits::CommandArg;

pub fn arguments() -> Vec<Box<dyn CommandArg>> {
    vec![
        Box::new(checksum::ChecksumArgument::new()),
        Box::new(parse::ParseArgument::new()),
    ]
}
