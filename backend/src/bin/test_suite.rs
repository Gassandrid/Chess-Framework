use chess_engine_api::engine::test_suite::run_all_tests;

fn main() {
    println!("Chess Engine Test Suite Runner");
    println!("===============================\n");

    // Run all tests with depth 6 for tactical positions
    run_all_tests(6);
}
