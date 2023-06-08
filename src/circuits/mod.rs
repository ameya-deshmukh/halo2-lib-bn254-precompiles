pub mod bn254_table; //contains the precompile table for the various operations. 

//For more details on the precompile table design, please refer to the code documentation in bn254_table.rs

pub mod bn254_circuit_chip; //contains the subcircuit implementation, with it being abstracted as 'Bn254Chip'

pub mod bn254_circuit_raw; //contains the subcircuit implementation without abstracting it away as a Chip