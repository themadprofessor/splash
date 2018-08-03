mod photos;

use std::error::Error;
use std::fmt;

pub use self::photos::Photos;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Order {
    Latest,
    Oldest,
    Popular
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Errors(Vec<String>);

impl Error for Errors {}

impl fmt::Display for Errors {
    fn fmt(&self, f: & mut fmt::Formatter) -> Result<(), fmt::Error> {
        for e in self.0.iter() {
            f.write_str(&e)?;
        }
        Ok(())
    }
}

impl Default for Order {
    fn default() -> Self {
        Order::Latest
    }
}
