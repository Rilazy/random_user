//!
//! This library provides a convenient random user generator using the api from <https://randomuser.me>
//!
//! Built with [reqwest](https://docs.rs/reqwest/latest/reqwest/) and using async.
//!
//! ## Examples
//!
//! ### Generate one random user:
//! ```
//! use random_user::UserGenerator;
//!
//! #[tokio::main]
//! async fn main() {
//!     let generator = UserGenerator::new();
//!
//!     let user = generator
//!         .fetch_one()
//!         .await
//!         .expect("Error fetching random user");
//!
//!     println!("{:#?}", user);
//! }
//! ```
//!
//! ### Generate multiple random users with filters:
//! ```
//! use random_user::{Nationality, Gender, UserGenerator};
//! #[tokio::main]
//! async fn main() {
//!     let generator = UserGenerator::new();
//!     // Get 5 Austrialian women with complex passwords
//!     let users = generator
//!         .get()
//!         .gender(Gender::Female)
//!         .nationality(Nationality::Australian)
//!         .password("upper,lower,special,12-24")
//!         .fetch(5)
//!         .await;
//!         
//! }
//! ```

mod generator;
mod types;

pub use generator::UserGenerator;
pub use types::*;
