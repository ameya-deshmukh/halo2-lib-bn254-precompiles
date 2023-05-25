//BN254 table design and implementation 

use halo2_ecc::{fields::fp, ecc::{EcPoint, EccChip}};

use serde::{Deserialize, Serialize};
/*

BN254 precompile table 

Proved by the BN254 subcircuit

This is a 'virtual' table within the BN254 circuit assignments - for the four operations, ecRecover, ecAdd, ecMul and ecPairing.

The table structure is as follows:

|id | tag | input_length | output_length | input | output |
|---|-----|--------------|---------------|-------|--------|
|---|-----|--------------|---------------|-------|--------|


where

id: is the ID corresponding to the bn254 operation (e.g. 0 for ecRecover, 1 for ecAdd and so on)

tag: is the tag corresponding to the bn254 operation (e.g. 'ecRecover', 'ecAdd' etc. )
the tag also acts like a 'selector' of sorts for the operation


input_length: is the length of the input in bytes

output_length: is the length of the output in bytes

input: is the input for a bn254 operation

output: is the output for a bn254 operation


 */

//helper structs, enums and traits
//might be needed later

/* #[derive(Debug, Clone)]
struct Expression{}

impl Expression{

fn expr(&self) -> fp {
}

fn eq(&self, other: &self)-> bool {
    self.expr() == other.expr()
}

} */


//Tag for BN254 lookup
pub enum Bn254OperationTag {

ECRECOVER=0,
ECADD = 1,
ECMUL=2,
ECPAIRING=3,

}

//Table row struct
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bn254TableRow{
    id: String,
    tag: String,
    input1: EcPoint,
    input2: EcPoint,
    output: EcPoint
}

//todo: implement with generalized lookup function to be compatible with the zkEVM specs
pub fn bn254_lookup(&mut bn254_table: Vec<Bn254TableRow> , 
    id: String, 
    tag: String, 
    input1: EcPoint,
    input2: EcPoint,
    output: EcPoint,) -> Option<Bn254TableRow, usize> {

    for (index, row) in bn254_table.iter().enumerate() {
        if row.id == id && row.tag == tag && row.input1 == input1 && row.input2 == input2 && row.output == output {
            return Some((row.clone(), index));
        }
    }
None
} 

//tests 
