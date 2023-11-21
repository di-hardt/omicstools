# Omictools

## Constant data
Constant data like element masses are provided as CSV-file and gets compiled on build time (see: build.rs).
* Amino acid data is used from: https://proteomicsresource.washington.edu/protocols06/masses.php
* Elements data is used from: https://proteomicsresource.washington.edu/protocols06/masses.php
* Subatomic particles data is used from en.wikipedia.org/wiki/(Proton|Electron|Neutron) (need better source)

## Installation
`cargo add dihardts_omicstools`

## Development

### Dependencies
* Rust
* OpenSSL (Ubuntu: `libssl-dev`)

### Unit tests
* For testing the taxonomy reader a [`taxdmp.zip`](https://ftp.ncbi.nih.gov/pub/taxonomy/taxdmp.zip) is required. The test is capable of download it by itself and save it to the tmp folder, however if you plan excessive testing over a couple of reboots, download it ones and set the environment variable `TAXDMP_ZIP_PATH` with the path to the file.