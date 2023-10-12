use std::error::Error;

#[cfg_attr(test, mockall::automock)]
pub trait Connection {
    fn connect(&self) -> Result<(), Box<dyn Error>>;
    fn disconnect(&self) -> Result<(), Box<dyn Error>>;
    fn read(&self) -> Result<String, Box<dyn Error>>;
    fn write(&self, data: &str) -> Result<(), Box<dyn Error>>;
}
