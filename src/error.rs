use std::error::Error;

#[derive(Debug)]
pub struct HandlingError;

// impl Debug for HandlingError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Ok(())
//     }
// }
//
// impl Display for HandlingError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Ok(())
//     }
// }
//
// impl Error for HandlingError {
//     fn description(&self) -> &str {
//         "Error handling login information"
//     }
// }

#[derive(Debug)]
pub(crate) struct LoginError;

// impl Debug for LoginError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Ok(())
//     }
// }
//
// impl Display for LoginError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Ok(())
//     }
// }
//
// impl Error for LoginError {
//     fn description(&self) -> &str {
//         "Failed to log into the device."
//     }
// }