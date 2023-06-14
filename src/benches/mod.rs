use criterion::{criterion_group, criterion_main, Criterion, black_box};
use crate::hunter::{ContainsConsecutiveCharsWindowRule, ContainsConsecutiveCharsRegexRule, ContainsConsecutiveCharsCounterRule, Rule};

#[allow(dead_code)]
fn benchmark_rule(rule: &impl Rule, input: &str, criterion: &mut Criterion, name: &str) {
    let function_name = format!("{} {}", name, "benchmark");
    criterion.bench_function(&function_name, |b| {
        b.iter(|| rule.apply(black_box(&input.to_string())))
    });
}
#[allow(dead_code)]
fn counter_rule_benchmark(c: &mut Criterion) {
    let counter_rule = ContainsConsecutiveCharsCounterRule::new(3);
    benchmark_rule(&counter_rule, "Test string", c, "Counter rule");
}
#[allow(dead_code)]
fn window_rule_benchmark(c: &mut Criterion) {
    let window_rule = ContainsConsecutiveCharsWindowRule::new(3);
    benchmark_rule(&window_rule, "Test string", c, "Window rule");
}
#[allow(dead_code)]
fn regex_rule_benchmark(c: &mut Criterion) {
    let regex_rule = ContainsConsecutiveCharsRegexRule::new(3);
    benchmark_rule(&regex_rule, "Test string", c, "Regex rule");
}

criterion_group!(benches, counter_rule_benchmark, window_rule_benchmark, regex_rule_benchmark);
criterion_main!(benches);
