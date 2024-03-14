use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, Clone)]
pub struct Password;

impl Password {
    pub fn generate_random() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }

    pub fn hash(password: String) -> String {
        hash(password, DEFAULT_COST).unwrap()
    }

    pub fn generate_random_hash() -> String {
        let random_pass = Self::generate_random();
        let random_pass_hash = Self::hash(random_pass);
        return random_pass_hash;
    }
}
