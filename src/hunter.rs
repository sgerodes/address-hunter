use crate::address_manager::KeyBundle;
//use regex::Regex;
use fancy_regex::Regex;

pub trait Rule {
    fn apply(&self, key_bundle: &String) -> bool;
}

// pub struct MetamaskStartEndRule;

// impl Rule for MetamaskStartEndRule {
//     fn apply(&self, key_bundle: &KeyBundle) -> bool {
//         // implement your logic here, for example
//         false
//     }
// }

pub struct ContainsConsecutiveCharsRule {
    consecutive_chars_amount: usize,
}

impl ContainsConsecutiveCharsRule {
    pub fn new(consecutive_chars_amount: usize) -> Self {
        Self {
            consecutive_chars_amount,
        }
    }
}

impl Rule for ContainsConsecutiveCharsRule {
    fn apply(&self, public_address_no_0x: &String) -> bool {
        public_address_no_0x.as_bytes()
            .windows(self.consecutive_chars_amount)
            .any(|window| window.iter().all(|&ch| ch == window[0]))
    }
}

pub struct ContainsConsecutiveCharsRegexRule {
    regex: Regex,
}

impl ContainsConsecutiveCharsRegexRule {
    pub fn new(consecutive_chars_amount: usize) -> Self {
        let hex_chars = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];
        let mut pattern = String::new();
        for hex_char in hex_chars {
            let repeated_char = hex_char.repeat(consecutive_chars_amount);
            if !pattern.is_empty() {
                pattern.push('|');
            }
            pattern.push_str(&repeated_char);
        }
        let regex = Regex::new(&pattern).unwrap();
        Self {
            regex,
        }
    }
}

impl Rule for ContainsConsecutiveCharsRegexRule {
    fn apply(&self, public_address_no_0x: &String) -> bool {
        self.regex.is_match(public_address_no_0x).unwrap()
    }
}


pub struct ContainsConsecutiveCharsCounterRule {
    consecutive_chars_amount: usize,
}

impl ContainsConsecutiveCharsCounterRule {
    pub fn new(consecutive_chars_amount: usize) -> Self {
        Self {
            consecutive_chars_amount
        }
    }
}

impl Rule for ContainsConsecutiveCharsCounterRule {
    fn apply(&self, public_address_no_0x: &String) -> bool {
        let mut counter = 0;
        let mut last_char = ' ';
        for c in public_address_no_0x.chars() {
            if c == last_char {
                counter += 1;
                if counter >= self.consecutive_chars_amount {
                    return true;
                }
            } else {
                counter = 1;
            }
            last_char = c;
        }
        false
    }
}

// pub fn is_interesting(rules: Vec<Box<dyn Rule>>, key_bundle: KeyBundle) -> bool {
//     for rule in rules {
//         if rule.apply(&key_bundle) {
//             return true;
//         }
//     }
//     false
// }

// Usage
fn main() {
    // let bundle = KeyBundle::new(); // You would need to get or create a KeyBundle somehow
    // let rules: Vec<Box<dyn Rule>> = vec![
    //     Box::new(MetamaskStartEndRule),
    //     Box::new(ContainsConsecutiveCharsRule),
    //     // add more rules as needed
    // ];
    // let result = is_interesting(rules, bundle);
    // println!("Result: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consecutive_rule_test() {
        let rule = ContainsConsecutiveCharsRule::new(7);

        let should_pass = [
            "55555552af5e170c3ec77a1233883c8a7e8a827f",
            "12545552af5e170c3ec77a1233883c8a75555555",
            "12545552af5e155555557a1233883c8a75444455",
        ];

        let should_fail = [
            "55555532af5e170c3ec77a1233883c8a7e8a827f",
            "12545552af5e170c3ec77a1233883c8a75555551",
            "12545552af5e155555537a1233883c8a75444455",
        ];
        
        for sp in should_pass.iter() {
            assert!(rule.apply(&sp.to_string()));
        }
        for sf in should_fail.iter() {
            assert!(!rule.apply(&sf.to_string()));
        }
    }

    #[test]
    fn consecutive_regex_rule_test() {
        let rule = ContainsConsecutiveCharsRegexRule::new(7);

        let should_pass = [
            "55555552af5e170c3ec77a1233883c8a7e8a827f",
            "12545552af5e170c3ec77a1233883c8a75555555",
            "12545552af5e155555557a1233883c8a75444455",
        ];

        let should_fail = [
            "55555532af5e170c3ec77a1233883c8a7e8a827f",
            "12545552af5e170c3ec77a1233883c8a75555551",
            "12545552af5e155555537a1233883c8a75444455",
        ];
        
        for sp in should_pass.iter() {
            assert!(rule.apply(&sp.to_string()));
        }
        for sf in should_fail.iter() {
            print!("{} ", sf);
            assert!(!rule.apply(&sf.to_string()));
        }
    }

    #[test]
    fn consecutive_counter_rule_test() {
        let rule = ContainsConsecutiveCharsCounterRule::new(7);

        let should_pass = [
            "55555552af5e170c3ec77a1233883c8a7e8a827f",
            "12545552af5e170c3ec77a1233883c8a75555555",
            "12545552af5e155555557a1233883c8a75444455",
        ];

        let should_fail = [
            "55555532af5e170c3ec77a1233883c8a7e8a827f",
            "12545552af5e170c3ec77a1233883c8a75555551",
            "12545552af5e155555537a1233883c8a75444455",
        ];
        
        for sp in should_pass.iter() {
            assert!(rule.apply(&sp.to_string()));
        }
        for sf in should_fail.iter() {
            print!("{} ", sf);
            assert!(!rule.apply(&sf.to_string()));
        }
    }
}
