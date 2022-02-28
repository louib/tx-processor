# tx-processor
![Tests status](https://github.com/louib/tx-processor/workflows/tests/badge.svg)
![Code formatting](https://github.com/louib/tx-processor/workflows/formatting/badge.svg)

Simple transaction processor in Rust.

## Implementation details
* Errors encountered when processing transactions are currently logged to `stderr`, but are not acted upon, or logged
  to a log file for inspection.
* Errors in the formatting of the CSV file containing the transactions are currently not handled.
* The records in the transactions CSV file are read using [`csv::Reader::deserialize`](https://docs.rs/csv/1.1.6/csv/struct.Reader.html#method.deserialize).
  The buffer size used by the `csv::Reader` can be configured using the [`buffer_capacity`](https://docs.rs/csv/1.1.6/csv/struct.Reader.html#method.deserialize)
  function, but the default value is being used at the moment. Benchmarking could determine if a different value is more appropriate.
* The transaction processing functions are currently not thread-safe.
* Atomicity is currently not guaranteed during transaction processing, but the critical sections have been identified in the code.
* Decimal precision is currently configured to 4 four digits, but can be changed using the `DECIMAL_PRECISION` const.
* At the moment, accounts that are locked will no longer process transactions.

## Usage
```
tx-processor transactions.csv
```

The state of all the accounts will be printed to `stdout`.
Errors encountered while processing transactions will be printed to `stderr`.
