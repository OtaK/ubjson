# ubjson

Rust UBJSON parser made with `nom`

It's decently fast.

## Roadmap

- [x] Implement parsing with `nom`
- [ ] Implement serialization
- [ ] Add support for `serde`

## Benchmarks

Feel free to run `cargo bench` yourself. It compares `ubjson` with `serde_json`.

<details><summary>Results on a i7 4770K @ 4.2GHz running Linux Kernel 5.9.6</summary>
<p>

* ubjson 0.1.0
* serde/serde_json 1.0.x

```

     Running target/release/deps/ubjson-799c5ad13c99f8cc
parse_complex_couchdb   time:   [14.271 us 14.280 us 14.290 us]
                        change: [+1.6967% +1.8314% +1.9554%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe

parse_complex_media     time:   [3.8222 us 3.8240 us 3.8269 us]
                        change: [-14.404% -14.257% -14.134%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

parse_complex_twitter   time:   [9.0176 us 9.0332 us 9.0518 us]
                        change: [-15.120% -14.943% -14.758%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) high mild
  13 (13.00%) high severe

     Running target/release/deps/vs_serde_json-a19f033dbf3cecab
parse_complex_couchdb/ubjson
                        time:   [13.945 us 13.948 us 13.952 us]
                        thrpt:  [217.63 MiB/s 217.70 MiB/s 217.75 MiB/s]
                 change:
                        time:   [-5.6706% -5.3791% -5.0618%] (p = 0.00 < 0.05)
                        thrpt:  [+5.3317% +5.6849% +6.0115%]
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe
parse_complex_couchdb/serde_json
                        time:   [16.137 us 16.148 us 16.160 us]
                        thrpt:  [187.90 MiB/s 188.05 MiB/s 188.16 MiB/s]
                 change:
                        time:   [-2.6257% -1.8876% -1.2836%] (p = 0.00 < 0.05)
                        thrpt:  [+1.3003% +1.9239% +2.6965%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe

parse_complex_media/ubjson
                        time:   [3.9193 us 3.9227 us 3.9270 us]
                        thrpt:  [107.10 MiB/s 107.21 MiB/s 107.31 MiB/s]
                 change:
                        time:   [-14.435% -13.498% -12.709%] (p = 0.00 < 0.05)
                        thrpt:  [+14.559% +15.605% +16.871%]
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
parse_complex_media/serde_json
                        time:   [3.7627 us 3.7683 us 3.7749 us]
                        thrpt:  [111.41 MiB/s 111.61 MiB/s 111.77 MiB/s]
                 change:
                        time:   [-4.8917% -3.5596% -2.7267%] (p = 0.00 < 0.05)
                        thrpt:  [+2.8031% +3.6910% +5.1433%]
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

parse_complex_twitter/ubjson
                        time:   [9.1501 us 9.1640 us 9.1806 us]
                        thrpt:  [186.67 MiB/s 187.01 MiB/s 187.29 MiB/s]
                 change:
                        time:   [-17.787% -17.131% -16.649%] (p = 0.00 < 0.05)
                        thrpt:  [+19.974% +20.672% +21.635%]
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
parse_complex_twitter/serde_json
                        time:   [11.744 us 11.765 us 11.790 us]
                        thrpt:  [145.36 MiB/s 145.67 MiB/s 145.92 MiB/s]
                 change:
                        time:   [-9.0925% -7.3039% -5.9318%] (p = 0.00 < 0.05)
                        thrpt:  [+6.3058% +7.8794% +10.002%]
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

```
</p>
</details>

## License

Licensed under either of these:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
   [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))
