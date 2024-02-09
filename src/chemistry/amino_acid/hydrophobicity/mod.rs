/// Krokhin hydrophobicity V1 & V3
///
/// > An Improved Model for Prediction of Retention Times of Tryptic Peptides in Ion Pair Reversed-phase HPLC: Its Application to Protein Peptide   
/// > Krokhin, O. V.; Craig, R.; Spicer, V.; Ens, W.; Standing, K. G.; Beavis, R. C.; Wilkins, J. A.   
/// > Mol. Cell. Proteomics2004, 3, 908−919.   
/// > https://doi.org/10.1074/mcp.M400031-MCP200
///
/// > Sequence-Specific Retention Calculator. Algorithm for Peptide Retention Prediction in Ion-Pair RP-HPLC:  Application to 300- and 100-Å Pore Size C18 Sorbents   
/// > Oleg V. Krokhin   
/// > Analytical Chemistry 2006 78 (22), 7785-7795   
/// > https://doi.org/10.1021/ac060777w
///
/// According to bio.tools, the algorithm should be available at:
/// hs2.proteome.ca/SSRCalc/SSRCalc.html
///
/// A javascript implementation of the algorithm is available at:
/// https://downloads.thermofisher.com/assets/apps/peptide-analyzer/ssrcalc3.js?v=5
///
pub mod krokhin_hydrophobicity;
