use crate::address::eth_wallet::Wallet;
use crate::address::polka_wallet::PolkaWallet;
use crate::address_utils::address_utils;

use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::cmp::{Ord, PartialOrd, Ordering};
use std::fmt;

pub trait Rule {
    fn apply(&self, address_no_prefix: &String) -> bool;
}

pub struct MetamaskStartEndRule;

impl Rule for MetamaskStartEndRule {
    fn apply(&self, address_no_prefix: &String) -> bool {
        // starts with 3, ends with 4 same
        let starting_count:usize = 3;
        let ending_count:usize = 4;
        let bytes = address_no_prefix.as_bytes();

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


pub struct PolkadotJsStartEndRule;

impl Rule for PolkadotJsStartEndRule {
    fn apply(&self, address_no_prefix: &String) -> bool {
        // starts with 3, ends with 4 same
        let starting_count:usize = 6;
        let ending_count:usize = 6;
        let bytes = address_no_prefix.as_bytes();

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

impl PolkadotJsStartEndRule {
    pub fn new() -> Self {
        Self {}
    }
}


pub struct StartRule<'a> {
    starting_words: &'a [&'a str],
    starting_words_lowercase: Vec<String>,
    case_sensitive: bool
}

impl<'a> StartRule<'a> {
    pub fn new(starting_words: &'a [&'a str], case_sensitive: bool) -> Self {
        let starting_words_low_vec: Vec<String> = starting_words.iter().map(|&w| w.to_lowercase()).collect();
        Self {
            starting_words,
            starting_words_lowercase: starting_words_low_vec,
            case_sensitive,
        }
    }
    fn matched_word(&self, address_no_prefix: &String) -> &str {
        for &word in self.starting_words {
            if address_no_prefix.starts_with(word) {
                return word;
            }
        }
        ""
    }

    fn matched_word_case_insensitive(&self, address_no_prefix: &String) -> &str {
        let address_no_prefix_lowercase = address_no_prefix.to_lowercase();
        for word in &self.starting_words_lowercase {
            if address_no_prefix_lowercase.starts_with(word) {
                return &word;
            }
        }
        ""
    }
}

impl<'a> Rule for StartRule<'a> {
    fn apply(&self, address_no_prefix: &String) -> bool {
        if self.case_sensitive {
            for &word in self.starting_words {
                if address_no_prefix.starts_with(word) {
                    return true;
                }
            }
        } else {
            let address_no_prefix_lowercase = address_no_prefix.to_lowercase();
            for word in &self.starting_words_lowercase {
                if address_no_prefix_lowercase.starts_with(word) {
                    return true;
                }
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
    fn apply(&self, address_no_prefix: &String) -> bool {
        address_no_prefix.as_bytes()
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
    fn apply(&self, address_no_prefix: &String) -> bool {
        self.regex.is_match(address_no_prefix).unwrap()
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
    fn apply(&self, address_no_prefix: &String) -> bool {
        let mut counter = 0;
        let mut last_char = ' ';
        for c in address_no_prefix.chars() {
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
    fn apply(&self, address_no_prefix: &String) -> bool {
        let bytes = address_no_prefix.as_bytes();
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
    fn apply(&self, address_no_prefix: &String) -> bool {
        let score = self.evaluate_vanity_quality(address_no_prefix);
        // Here you need to decide what score is considered "high quality"
        score > 1000000.0
    }
}


// entropy 

pub struct CharEntropyRule {
    entropy_coefficient_max_boundary: f64
}

impl Rule for CharEntropyRule {
    fn apply(&self, address_no_prefix: &String) -> bool {
        address_utils::calculate_entropy(address_no_prefix) <= self.entropy_coefficient_max_boundary
    }
}

impl CharEntropyRule {
    pub fn new(entropy_coefficient_max_boundary: f64) -> Self {
        Self {entropy_coefficient_max_boundary}
    }
}


// proximity coefficient


pub struct ProximityCoefficientRule {
    proximity_coefficient_min_boundary: f64,
    proximity_max_distance: usize
}

impl Rule for ProximityCoefficientRule {
    fn apply(&self, address_no_prefix: &String) -> bool {
        address_utils::calculate_proximity_coefficient(address_no_prefix, self.proximity_max_distance) >= self.proximity_coefficient_min_boundary
    }
}

impl ProximityCoefficientRule {
    pub fn new(proximity_coefficient_min_boundary: f64, proximity_max_distance: usize) -> Self {
        Self {proximity_coefficient_min_boundary, proximity_max_distance}
    }
}

// zero bytes

pub struct ZeroBytesRule{
    zero_bytes_count: usize,
}

impl Rule for ZeroBytesRule {
    fn apply(&self, address_no_prefix: &String) -> bool {
        self.count_zero_bytes(address_no_prefix) >= self.zero_bytes_count
    }
}

impl ZeroBytesRule {
    pub fn new(zero_bytes_count: usize) -> Self {
        Self {
            zero_bytes_count
        }
    }
    fn count_zero_bytes(&self, address_no_prefix: &String) -> usize {
        let bytes = address_no_prefix.as_bytes();
        let mut count = 0;
        for i in (0..bytes.len()).step_by(2) {
            if bytes[i] == b'0' && bytes[i + 1] == b'0' {
                count += 1;
            }
        }
        count
    }
}




lazy_static! {
    pub static ref METAMASK_RULE: MetamaskStartEndRule = MetamaskStartEndRule::new();
    pub static ref CONSECUTIVE_CHARS_RULE: ContainsConsecutiveCharsCounterRule = ContainsConsecutiveCharsCounterRule::new(9);
    pub static ref START_RULE: StartRule<'static> = StartRule::new(&["decaff", "facade", "c0ffee", "dec0de", "01234567", "12345678", "abcdef", "fedcba", "98765432"], true);
    pub static ref START_CONSECUTIVE_CHARS_RULE: StartsConsecutiveCharsCounterRule = StartsConsecutiveCharsCounterRule::new(7);
    pub static ref ZERO_BYTES_RULE: ZeroBytesRule = ZeroBytesRule::new(5);
    pub static ref CHAR_ENTROPY_RULE: CharEntropyRule = CharEntropyRule::new(2.8);
    pub static ref PROXIMITY_RULE: ProximityCoefficientRule = ProximityCoefficientRule::new(23.0, 3);
    pub static ref CHAR_ENTROPY_RULE_3: CharEntropyRule = CharEntropyRule::new(3.0);
    pub static ref PROXIMITY_RULE_3_21: ProximityCoefficientRule = ProximityCoefficientRule::new(21.0, 3);
}


lazy_static! {
    pub static ref POLKADOT_POLKADOT_JS: PolkadotJsStartEndRule = PolkadotJsStartEndRule::new();
    pub static ref POLKADOT_CONSECUTIVE_CHARS_RULE: ContainsConsecutiveCharsCounterRule = ContainsConsecutiveCharsCounterRule::new(5);
    pub static ref POLKADOT_START_RULE: StartRule<'static> = StartRule::new(&["012345", "123456", "abcdef", "dev", "crypto", "web3", "coin", "chain", "wallet", "star", "galaxy", "zen", "future", "byte", "quantum"], false);
    pub static ref POLKADOT_START_CONSECUTIVE_CHARS_RULE: StartsConsecutiveCharsCounterRule = StartsConsecutiveCharsCounterRule::new(4);
    pub static ref POLKADOT_CHAR_ENTROPY_RULE: CharEntropyRule = CharEntropyRule::new(2.8);
    pub static ref POLKADOT_PROXIMITY_RULE: ProximityCoefficientRule = ProximityCoefficientRule::new(23.0, 3);
}



pub struct VanityResult {
    pub wallet: Wallet,
    pub matched_rule: Option<String>,
    pub met_criteria: bool,
    pub entropy_coefficient: f64,
    pub proximity_coefficient: f64,
    pub entropy_coefficient_checksummed: f64,
    pub proximity_coefficient_checksummed: f64
}

pub struct PolkadotVanityResult {
    pub wallet: PolkaWallet,
    pub matched_rule: Option<String>,
    pub met_criteria: bool,
    pub entropy_coefficient: f64,
    pub proximity_coefficient: f64
}

impl fmt::Display for VanityResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VanityResult {{
    Matched Rule: {:?},
    Entropy Coefficient: {:.2},
    Proximity Coefficient: {:.2},
    Entropy Coefficient (Checksummed): {:.2},
    Proximity Coefficient (Checksummed): {:.2}
}}",
            self.matched_rule,
            self.entropy_coefficient,
            self.proximity_coefficient,
            self.entropy_coefficient_checksummed,
            self.proximity_coefficient_checksummed
        )
    }
}

impl fmt::Display for PolkadotVanityResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PolkadotVanityResult {{
    Matched Rule: {:?},
    Entropy Coefficient: {:.2},
    Proximity Coefficient: {:.2}
}}",
            self.matched_rule,
            self.entropy_coefficient,
            self.proximity_coefficient
        )
    }
}

pub fn does_polkadot_address_meet_criteria(wallet: &PolkaWallet) -> PolkadotVanityResult {
    let address_no_prefix = &wallet.address;

    let entropy_coefficient: f64 = address_utils::calculate_entropy(address_no_prefix);
    let proximity_coefficient: f64 = address_utils::calculate_proximity_coefficient(address_no_prefix, POLKADOT_PROXIMITY_RULE.proximity_max_distance);

    let mut matched_rule: Option<String> = None;
    let mut met_criteria = false;

    if POLKADOT_START_CONSECUTIVE_CHARS_RULE.apply(address_no_prefix) {
        let consecutive_chars = max_consecutive_chars(address_no_prefix);
        matched_rule = Some(format!("Start consecutive rule. Consecutive {}", consecutive_chars));
        met_criteria = true;
    }
    else if POLKADOT_POLKADOT_JS.apply(address_no_prefix) {
        let first_char = address_no_prefix.chars().nth(0).unwrap();
        let last_char = address_no_prefix.chars().rev().nth(0).unwrap();
        matched_rule = Some(format!("Polkadot js rule {}{}", first_char, last_char));
        met_criteria = true;
    }
    else if POLKADOT_CONSECUTIVE_CHARS_RULE.apply(address_no_prefix) { 
        let consecutive_chars = max_consecutive_chars(address_no_prefix);
        matched_rule = Some(format!("Consecutive rule. Consecutive {}", consecutive_chars));
        met_criteria = true;
    }
    else if POLKADOT_START_RULE.apply(address_no_prefix) {
        let word = START_RULE.matched_word(address_no_prefix);
        matched_rule = Some(format!("Start word rule. word {}", word));
        met_criteria = true;
    } 
    else if POLKADOT_CHAR_ENTROPY_RULE.apply(address_no_prefix) {
        matched_rule = Some(format!("Entropy < {}", CHAR_ENTROPY_RULE.entropy_coefficient_max_boundary));
        met_criteria = true;
    } 
    else if POLKADOT_PROXIMITY_RULE.apply(address_no_prefix) {
        matched_rule = Some(format!("Proximity {} > {}", PROXIMITY_RULE.proximity_max_distance, PROXIMITY_RULE.proximity_coefficient_min_boundary));
        met_criteria = true;
    }

    PolkadotVanityResult { 
        wallet: wallet.clone(), 
        matched_rule, 
        met_criteria,
        entropy_coefficient,
        proximity_coefficient
    }
}

pub fn does_address_meet_criteria(wallet: &Wallet) -> VanityResult {
    let address_no_prefix = &wallet.address;
    let address_checksummed = &wallet.address_checksummed;

    let entropy_coefficient: f64 = address_utils::calculate_entropy(address_no_prefix);
    let proximity_coefficient: f64 = address_utils::calculate_proximity_coefficient(address_no_prefix, PROXIMITY_RULE.proximity_max_distance);
    let entropy_coefficient_checksummed: f64 = address_utils::calculate_entropy(address_checksummed);
    let proximity_coefficient_checksummed: f64 = address_utils::calculate_proximity_coefficient(address_checksummed, PROXIMITY_RULE_3_21.proximity_max_distance);

    let mut matched_rule: Option<String> = None;
    let mut met_criteria = false;
    if ZERO_BYTES_RULE.apply(address_no_prefix) {
        let zero_bytes_count = ZERO_BYTES_RULE.count_zero_bytes(address_no_prefix);
        matched_rule = Some(format!("Zero Bytes Rule {}", zero_bytes_count));
        met_criteria = true;
    }
    else if address_checksummed.starts_with("DE") && (address_checksummed.ends_with("0001") || address_checksummed.ends_with("0000") || address_checksummed.ends_with("0002")) {
    //else if address_checksummed.starts_with("DE") && address_checksummed.ends_with("01") && (entropy_coefficient_checksummed <= 3.6 || proximity_coefficient_checksummed > 14.0)  {
        matched_rule = Some("DEV".parse().unwrap());
        met_criteria = true;
    }

    else if START_CONSECUTIVE_CHARS_RULE.apply(address_no_prefix) {
        let consecutive_chars = max_consecutive_chars(address_no_prefix);
        matched_rule = Some(format!("Start consecutive rule. Consecutive {}", consecutive_chars));
        met_criteria = true;
    }
    else if METAMASK_RULE.apply(address_no_prefix) {
        let first_char = address_no_prefix.chars().nth(0).unwrap();
        let last_char = address_no_prefix.chars().rev().nth(0).unwrap();
        matched_rule = Some(format!("Metamask rule {}{}", first_char, last_char));
        met_criteria = true;
    }
    else if CONSECUTIVE_CHARS_RULE.apply(address_no_prefix) { 
        let consecutive_chars = max_consecutive_chars(address_no_prefix);
        matched_rule = Some(format!("Consecutive rule. Consecutive {}", consecutive_chars));
        met_criteria = true;
    }
    else if START_RULE.apply(address_no_prefix) {
        let word = START_RULE.matched_word(address_no_prefix);
        matched_rule = Some(format!("Start word rule. word {}", word));
        met_criteria = true;
    } 
    else if CHAR_ENTROPY_RULE.apply(address_no_prefix) {
        matched_rule = Some(format!("Entropy < {}", CHAR_ENTROPY_RULE.entropy_coefficient_max_boundary));
        met_criteria = true;
    } 
    else if PROXIMITY_RULE.apply(address_no_prefix) {
        matched_rule = Some(format!("Proximity {} > {}", PROXIMITY_RULE.proximity_max_distance, PROXIMITY_RULE.proximity_coefficient_min_boundary));
        met_criteria = true;
    } 
    else if CHAR_ENTROPY_RULE_3.apply(address_checksummed) {
        matched_rule = Some(format!("Checksummed Entropy < {}", CHAR_ENTROPY_RULE_3.entropy_coefficient_max_boundary));
        met_criteria = true;
    }
    else if PROXIMITY_RULE_3_21.apply(address_checksummed) {
        matched_rule = Some(format!("Checksummed Proximity {} > {}", PROXIMITY_RULE_3_21.proximity_max_distance, PROXIMITY_RULE_3_21.proximity_coefficient_min_boundary));
        met_criteria = true;
    }

    

    VanityResult { 
        wallet: wallet.clone(), 
        matched_rule, 
        met_criteria,
        entropy_coefficient,
        proximity_coefficient,
        entropy_coefficient_checksummed,
        proximity_coefficient_checksummed
    }
}

fn max_consecutive_chars(s: &str) -> usize {
    let mut max_char = ' ';
    let mut max_count = 0;
    let mut last_char = ' ';
    let mut last_count = 0;

    for c in s.chars() {
        if c == last_char {
            last_count += 1;
        } else {
            last_char = c;
            last_count = 1;
        }

        if last_count > max_count {
            max_char = c;
            max_count = last_count;
        }
    }
    max_count
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
        let rule = StartRule::new(&["decaff", "facade", "c0ffee", "dec0de", "01234567", "12345678", "abcdef", "fedcba", "98765432"], true);
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
