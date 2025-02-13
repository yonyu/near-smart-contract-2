// Find all our documentation at https://docs.near.org
use near_sdk::{log, near};
use near_sdk::env;

const PASSWORD_NUMBER: u8 = 1;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    // Set up contract state
    greeting: String,
    password_solution: String
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
            password_solution: "".to_string()
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {

    #[init]
    pub fn new(password_solution: String) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            greeting: "Hello".to_string(),
            password_solution: password_solution
        }
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {greeting}");
        self.greeting = greeting;
    }
    
    pub fn get_solution(&self) -> String {
        self.password_solution.clone()
    }

    pub fn get_password_number(&self) -> u8 {
        PASSWORD_NUMBER
    }

    pub fn set_password_solution(&mut self, solution: String) {
        log!("Saving password solution: {solution}");
        self.password_solution = solution;
    }

    pub fn guess_password(&mut self, solution: String) {
        if solution == self.password_solution {
            env::log_str("Password is correct!");
        } else {
            env::log_str("Password is incorrect!");
        }
    }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);
        if hashed_input_hex == self.password_solution {
            env::log_str("You may enter! This is the right password!");
            return true;
        } else {
            env::log_str("You should not pass. Please try again.");
            return false;
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::get_logs;
    //use near_sdk::borsh::de;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::AccountId;

    // Part of writing tests is setting up a mock context
    // Providing a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);

        builder
    }

    // Debugging and iteration of a unit test
    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "Kenneth Yu";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);

        println!("Let's debug: {:?}", debug_hash_string);
    }

    #[test]
    fn check_guess_solution() {
        let account_id = "ypyu.testnet".parse().unwrap();
        let context = get_context(account_id);
        testing_env!(context.build());

        let mut contract = Contract::new("94e41df9f79737cd713d82612db5f31e9a5c84f8fe645ad211ae8e07f071a2d6".to_string());

        let mut guess_result = contract.guess_solution("wrong answer".to_string());
        assert!(!guess_result, "Password is incorrect!");
        assert_eq!(get_logs(), ["You should not pass. Please try again."], "Expecting password is incorrect!");

        guess_result = contract.guess_solution("Kenneth Yu".to_string());
        assert!(guess_result, "This is the correct answer!");
        assert_eq!(get_logs(), ["You should not pass. Please try again.", "You may enter! This is the right password!"], "Expecting password is correct!");
    }

    #[test]
    fn debug_get_context() {
        let account_id = "ypyu.testnet".parse().unwrap();
        let build = get_context(account_id);
        assert_eq!(build.context.predecessor_account_id, "ypyu.testnet".to_string());
    }

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }
}

