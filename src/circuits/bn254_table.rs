
/*BN254 precompile table design and implementation

'Proved' by the BN254 subcircuit implementation 

The structure of the table is as follows:

sequence_id |  operation | io | length | index | byte 
------------| -----------|----|--------|-------|------
------------| -----------|----|--------|-------|------

sequence_id: identifies the precompile call in an execution trace 
operation: identifies the operation type (e.g. ecAdd, ecMul, ecPairing etc.)
io: identifies an input (1) or output (0)
length: identifies the length of the 
index: identifies the index of the input/output in the execution trace - ranges from 0 to length-1
byte: hex data of the input/output, 1 byte 

*/
#[derive(Debug, Clone)]
pub enum OpTag{

    #[allow(non_camel_case_types)]
    ecRecover=0,
    ecAdd,
    ecMul,
    ecPairing
}

#[derive(Debug, Clone)]
pub struct Bn254TableRow{
    pub seq_id: usize,
    pub operation: OpTag,
    pub length: usize,
    pub io: bool,
    pub index: usize, 
    pub byte: u8
} 

impl Bn254TableRow{
    
    pub fn new(seq_id: usize, operation: OpTag, length: usize, io: bool, index: usize, byte: u8) -> Self{
            Self{
                seq_id,
                operation,
                length,
                io, 
                index,
                byte
            }
    }

    pub fn add_row(bn254table: &mut Vec<Self>, new_row: Bn254TableRow){
        bn254table.push(new_row);
    }

} 
