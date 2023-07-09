use fancy_regex::Regex;
use lazy_static::lazy_static;


pub trait Rule {
    fn apply(&self, public_address_no_0x: &String) -> bool;
}

pub struct MetamaskStartEndRule;

impl Rule for MetamaskStartEndRule {
    fn apply(&self, public_address_no_0x: &String) -> bool {
        // starts with 3, ends with 4 same
        let starting_count:usize = 3;
        let ending_count:usize = 4;
        let bytes = public_address_no_0x.as_bytes();

        for s in 1..starting_count {
            if bytes[s] != bytes[0] {
                return false;
            }
        }
        for f in 1..(ending_count+1) {
            if bytes[bytes.len() - f] != bytes[bytes.len() - 1] {
                return false;
            }
        }
    
        true
    }
}

impl MetamaskStartEndRule {
    pub fn new() -> Self {
        Self {}
    }
}
pub struct StartRule<'a> {
    starting_words: &'a [&'a str]
}

impl<'a> StartRule<'a> {
    pub fn new(starting_words: &'a [&'a str]) -> Self {
        Self {
            starting_words
        }
    }
}

impl<'a> Rule for StartRule<'a> {
    fn apply(&self, public_address_no_0x: &String) -> bool {
        for &word in self.starting_words {
            if public_address_no_0x.starts_with(word) {
                return true;
            }
        }
        false
    }
}

pub struct ContainsConsecutiveCharsWindowRule {
    consecutive_chars_amount: usize,
}

impl ContainsConsecutiveCharsWindowRule {
    pub fn new(consecutive_chars_amount: usize) -> Self {
        Self {
            consecutive_chars_amount,
        }
    }
}

impl Rule for ContainsConsecutiveCharsWindowRule {
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

pub struct StartsConsecutiveCharsCounterRule {
    consecutive_chars_amount: usize,
}

impl StartsConsecutiveCharsCounterRule {
    pub fn new(consecutive_chars_amount: usize) -> Self {
        Self {
            consecutive_chars_amount
        }
    }
}

impl Rule for StartsConsecutiveCharsCounterRule {
    fn apply(&self, public_address_no_0x: &String) -> bool {
        let bytes = public_address_no_0x.as_bytes();
        for s in 1..self.consecutive_chars_amount {
            if bytes[s] != bytes[0] {
                return false;
            }
        }
        true
    }
}

pub struct CharCounterRule {
}

use std::collections::HashMap;

impl CharCounterRule {
    pub fn new() -> Self {
        Self {
           
        }
    }
    fn count_chars(&self, s: &str) -> HashMap<char, usize> {
        let mut map = HashMap::new();
        for ch in s.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }
        map
    }

    // Compute a HashMap with each character in the string and its probability of occurrence
    fn compute_char_probabilities(&self, s: &str) -> HashMap<char, f64> {
        let mut counts = HashMap::new();
        let total_chars = s.len() as f64;
        
        // Count occurrences of each character
        for ch in s.chars() {
            *counts.entry(ch).or_insert(0.0) += 1.0;
        }

        // Compute probabilities
        for value in counts.values_mut() {
            *value /= total_chars;
        }

        counts
    }

    // Compute the entropy of the string
    fn compute_entropy(&self, s: &str) -> f64 {
        let probabilities = self.compute_char_probabilities(s);

        // Compute entropy
        probabilities.values().fold(0.0, |acc, &p| {
            acc - p * p.log2()
        })
    }

    // Evaluate the vanity quality of an address: lower entropy means higher vanity quality
    fn evaluate_vanity_quality(&self, address: &str) -> f64 {
        -self.compute_entropy(address)
    }
    
}

impl Rule for CharCounterRule {
    fn apply(&self, public_address_no_0x: &String) -> bool {
        let score = self.evaluate_vanity_quality(public_address_no_0x);
        // Here you need to decide what score is considered "high quality"
        score > some_threshold
    }
}


impl Rule for CharEntropyRule {

    fn apply(&self, public_address_no_0x: &String) -> bool {

        false
    }
}


// static metamask_rule: MetamaskStartEndRule = MetamaskStartEndRule::new();
// static consecutive_chars_rule: ContainsConsecutiveCharsCounterRule = ContainsConsecutiveCharsCounterRule::new(7);
// static start_rule: StartRule = StartRule::new(&["decaff", "facade", "c0ffee", "dec0de", "01234567", "12345678", "abcdef", "fedcba", "98765432"]);

lazy_static! {
    pub static ref METAMASK_RULE: MetamaskStartEndRule = MetamaskStartEndRule::new();
    pub static ref CONSECUTIVE_CHARS_RULE: ContainsConsecutiveCharsCounterRule = ContainsConsecutiveCharsCounterRule::new(9);
    pub static ref START_RULE: StartRule<'static> = StartRule::new(&["decaff", "facade", "c0ffee", "dec0de", "01234567", "12345678", "abcdef", "fedcba", "98765432"]);
    pub static ref START_CONSECUTIVE_CHARS_RULE: StartsConsecutiveCharsCounterRule = StartsConsecutiveCharsCounterRule::new(7);
}

pub fn does_address_meet_criteria(public_address_no_0x: &String) -> bool {
    if START_CONSECUTIVE_CHARS_RULE.apply(public_address_no_0x) {
        print!("Start consecutive chars rule match found: {}\n", public_address_no_0x);
        return true;
    }
    if METAMASK_RULE.apply(public_address_no_0x) {
        print!("Metamask rule match found: {}\n", public_address_no_0x);
        return true;
    }
    if CONSECUTIVE_CHARS_RULE.apply(public_address_no_0x) { 
        print!("Consecutive chars rule match found: {}\n", public_address_no_0x);
        return true;
    }
    if START_RULE.apply(public_address_no_0x) {
        print!("Start rule match found: {}\n", public_address_no_0x);
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    const PERFORMANCE_TEST_ITERATIONS: i32 = 1000;

    fn iter_test(should_pass: &[&str], should_fail: &[&str], rule: &dyn Rule) {
        for sp in should_pass.iter() {
            assert!(rule.apply(&sp.to_string()));
        }
        for sf in should_fail.iter() {
            assert!(!rule.apply(&sf.to_string()));
        }
    }

    #[test]
    fn count_execution_time_of_consecutive_rules_test() {
        let rules: Vec<Box<dyn Rule>> = vec![
            Box::new(ContainsConsecutiveCharsCounterRule::new(7)),
            Box::new(ContainsConsecutiveCharsRegexRule::new(7)),
            Box::new(ContainsConsecutiveCharsWindowRule::new(7)),
        ];
    
        let test_cases = [
            "55555552af5e170c3ec77a1233883c8a7e8a827f",
            "12545552af5e170c3ec77a1233883c8a75555555",
            "12545552af5e155555557a1233883c8a75444455",
            "55555532af5e170c3ec77a1233883c8a7e8a827f",
            "12545552af5e170c3ec77a1233883c8a75555551",
            "12545552af5e155555537a1233883c8a75444455",
        ];
    
        for (i, rule) in rules.iter().enumerate() {
            let start = Instant::now();
            for _num in 0..PERFORMANCE_TEST_ITERATIONS {
                for case in test_cases.iter() {
                    rule.apply(&case.to_string());
                }
            }
            let duration = start.elapsed();
            println!("Time elapsed in Rule {}: {:?}", i+1, duration);
        }
    }

    #[test]
    fn consecutive_rule_test() {
        let rule = ContainsConsecutiveCharsWindowRule::new(7);

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
        iter_test(&should_pass, &should_fail, &rule);
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
        iter_test(&should_pass, &should_fail, &rule);
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
        iter_test(&should_pass, &should_fail, &rule);
    }

    #[test]
    fn metamask_rule_test() {
        let rule = MetamaskStartEndRule;

        let should_pass = [
            "55555312af5e170c3ec77a1233883c8a7e444444",
            "11145552af5e170c3ec77a1233883c8a75555555",
            "22222552af5e155555557a1233883c8a75445555",
        ];

        let should_fail = [
            "55555532af5e170c3ec77a1233883c8a7e8a827f",
            "44445552af5e170c3ec77a1233883c8a75555551",
            "12545552af5e155555537a1233883c8a75444444",
            "33545552af5e155555537a1233883c8a75444444",
            "33345552af5e155555537a1233883c8a75231444",
            "33145552af5e155555537a1233883c8a75234444",
        ];
        iter_test(&should_pass, &should_fail, &rule);
    }

    #[test]
    fn start_rule_test() {
        let rule = StartRule::new(&["decaff", "facade", "c0ffee", "dec0de", "01234567", "12345678", "abcdef", "fedcba", "98765432"]);
        let should_pass = [
            "decaff12af5e170c3ec77a1233883c8a7e444444",
            "c0ffee52af5e170c3ec77a1233883c8a75555555",
            "01234567af5e155555557a1233883c8a75445555",
        ];

        let should_fail = [
            "55555532af5e170c3ec77a1233883c8a7e8a827f",
            "44445552af5e170c3ec77a1233883c8a75555551",
            "12545552af5e155555537a1233883c8a75444444",
            "33545552af5e155555537a1233883c8a75444444",
        ];
        
        iter_test(&should_pass, &should_fail, &rule);
    }

}
