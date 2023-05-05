use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;


pub fn generate_random_password() -> String  {
    thread_rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect()
}