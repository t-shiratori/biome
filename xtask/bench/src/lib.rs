mod language;
mod test_case;

use crate::language::FormatNode;
pub use crate::language::Parse;
pub use crate::test_case::TestCase;
use biome_diagnostics::{DiagnosticExt, print_diagnostic_to_string};
use biome_formatter::Printed;
use biome_rowan::NodeCache;
use criterion::measurement::WallTime;
pub use criterion::*;

pub fn run_format(format_node: &FormatNode) -> Printed {
    let formatted = format_node.format_node().unwrap();
    let printed = formatted.print();
    drop(formatted);
    printed.expect("Document to be valid")
}

pub fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
    format!("{e:?}")
}

pub fn bench_parser_group(group: &mut BenchmarkGroup<WallTime>, test_case: TestCase) {
    let parse = Parse::try_from_case(&test_case).expect("Supported language");

    let code = test_case.code();
    let mut diagnostics = vec![];
    group.throughput(Throughput::Bytes(code.len() as u64));
    group.bench_with_input(
        BenchmarkId::new(test_case.filename(), "uncached"),
        &code,
        |b, _| {
            b.iter(|| {
                let result = black_box(parse.parse());
                diagnostics.extend(result.into_diagnostics());
            })
        },
    );
    for diagnostic in diagnostics {
        let diagnostic = diagnostic
            .with_file_source_code(code)
            .with_file_path(test_case.filename());
        println!("{}", print_diagnostic_to_string(&diagnostic));
    }
    group.bench_with_input(
        BenchmarkId::new(test_case.filename(), "cached"),
        &code,
        |b, _| {
            b.iter_batched(
                || {
                    let mut cache = NodeCache::default();
                    parse.parse_with_cache(&mut cache);
                    cache
                },
                |mut cache| {
                    black_box(parse.parse_with_cache(&mut cache));
                },
                BatchSize::SmallInput,
            )
        },
    );
}

pub fn bench_formatter_group(group: &mut BenchmarkGroup<WallTime>, test_case: TestCase) {
    let parse = Parse::try_from_case(&test_case).expect("Supported language");

    let code = test_case.code();

    group.throughput(Throughput::Bytes(code.len() as u64));
    group.bench_with_input(
        BenchmarkId::from_parameter(test_case.filename()),
        code,
        |b, _| {
            let parsed = parse.parse();

            match parsed.format_node() {
                None => {}
                Some(format_node) => b.iter(|| {
                    black_box(run_format(&format_node));
                }),
            }
        },
    );
}
