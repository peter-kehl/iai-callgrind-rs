use iai_callgrind_runner::api::EventKind;
use iai_callgrind_runner::runner::callgrind::model::Costs;
use iai_callgrind_runner::runner::callgrind::parser::Parser;
use iai_callgrind_runner::runner::callgrind::summary_parser::SummaryParser;
use iai_callgrind_runner::runner::callgrind::CallgrindStats;
use rstest::rstest;

use crate::common::get_callgrind_output;

// Ir Dr Dw I1mr D1mr D1mw ILmr DLmr DLmw
#[rstest]
#[case::no_records("no_records.with_summary_and_totals.out", [0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[case::with_records("no_entry_point.out", [325261, 78145, 35789, 1595, 2119, 850, 1558, 1485, 799])]
fn test_sentinel_parser(#[case] fixture: &str, #[case] costs: [u64; 9]) {
    let expected_stats = CallgrindStats(Costs::with_event_kinds([
        (EventKind::Ir, costs[0]),
        (EventKind::Dr, costs[1]),
        (EventKind::Dw, costs[2]),
        (EventKind::I1mr, costs[3]),
        (EventKind::D1mr, costs[4]),
        (EventKind::D1mw, costs[5]),
        (EventKind::ILmr, costs[6]),
        (EventKind::DLmr, costs[7]),
        (EventKind::DLmw, costs[8]),
    ]));
    let callgrind_output = get_callgrind_output(format!("callgrind.out/{fixture}"));

    let parser = SummaryParser;
    let actual_stats = parser.parse(&callgrind_output).unwrap();

    assert_eq!(actual_stats, expected_stats);
}

#[test]
fn test_summary_parser_when_not_found_then_error() {
    let callgrind_output =
        get_callgrind_output("callgrind.out/no_records.no_summary_and_totals.out");

    let parser = SummaryParser;
    assert_eq!(
        parser.parse(&callgrind_output).unwrap_err().to_string(),
        "Error parsing file 'tests/fixtures/callgrind.out/no_records.no_summary_and_totals.out': \
         No summary or totals line found"
            .to_owned()
    );
}