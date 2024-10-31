//! Ways to interact with data in external formats.
//!
//! There are two main ways to interact with the data:
//! - the SQL form (with associated functions in [critic::io::db])
//! - the flat form in raw text files (with associated functions in [critic::io::file])
//!
//! The end user should add data from flat files (which can easily be tracked in git), let critic
//! convert that data to SQL and can then query it (which is much easier once the data is SQL).
pub mod db;
pub mod file;
