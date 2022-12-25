# Running Tests Synchronously

Tests run in parallel by default. This can be dangerous, in certain circumstances. Imagine
a test that writes a file, then several tests that read from the file. If those tests are in
parallel, you may encounter race conditions. As a result, rust supports a flag to designate
the number of threads used to run the tests.

If you want to run some tests synchronously, use this command:
- `cargo test -- --test-threads=1`

This takes longer, but avoids possible race conditions.

# Run Subsets of Tests

We can also run a single test by specifying the test function name in the command:
- `cargo test should_panic`

Its possible to run multiple tests with a contains filter, like so:
- `cargo test this`

You can also specificy specific integration tests to run as follows:
- `cargo test --test sample_integration_test`