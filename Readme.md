# Omictools

## Constant data
For easier maintenance the data of some datatypes with multiple attributes like elements and amino acids is provided as CSV-file and gets compiled on build time (see: build.rs)
* Amino acid data is used from: <https://proteomicsresource.washington.edu/protocols06/masses.php>
* Elements data is used from: <https://proteomicsresource.washington.edu/protocols06/masses.php>
* Subatomic particles data is used from en.wikipedia.org/wiki/(Proton|Electron|Neutron) (need better source)
* Krokin retention coefficients
    * V1: <https://doi.org/10.1074/mcp.M400031-MCP200>
    * V3: <https://doi.org/10.1021/ac060777w>


Data with one or two attributes are directly added to the code:
* Amino acid hydropathicity_kd:
    > Kyte, J.; Doolittle, R. F.. A simple method for displaying the hydropathic character of a protein. Journal of molecular biology 1982, 157 (1), 105-32. <https://doi.org/10.1016/0022-2836(82)90515-0>.

## Installation
`cargo add dihardts_omicstools`

## Development

### Dependencies
* Rust
* OpenSSL (Ubuntu: `libssl-dev`)

### Unit tests
* For testing the taxonomy reader a [`taxdmp.zip`](https://ftp.ncbi.nih.gov/pub/taxonomy/taxdmp.zip) is required. The test is capable of download it by itself and save it to the tmp folder, however if you plan excessive testing over a couple of reboots, download it ones and set the environment variable `TAXDMP_ZIP_PATH` with the path to the file.