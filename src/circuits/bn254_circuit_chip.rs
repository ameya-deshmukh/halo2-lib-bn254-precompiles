
//BN254 subcircuit implementation - chip approach

#[allow(unused_imports)]

use std::env::var;

//halo2-base imports
use halo2_base::
    {AssignedValue, Context, 
    gates::{
        builder::{GateThreadBuilder, RangeCircuitBuilder},
        GateChip,
        RangeChip,
        GateInstructions,
        RangeInstructions
    }, utils::ScalarField,
};

use halo2_ecc::bigint::CRTInteger;
use halo2_ecc::ecc::fixed_base::scalar_multiply;
use halo2_ecc::fields::vector::FieldVector;
//halo2-ecc imports
use halo2_ecc::{
    bigint::{ProperCrtUint},
    bn254::{pairing::PairingChip, FpChip},
    ecc::{EcPoint, EccChip},
    fields::{FieldChip, PrimeField}
};



use halo2_base::halo2_proofs::halo2curves::bn256::{Fr, G1Affine, G2Affine};


//own imports
use super::bn254_table::*;

#[derive(Debug, Clone)]
pub struct OptagCircuitInput{
    pub op: OpTag, 
}

#[derive(Debug, Clone)]
pub struct AddCircuitInput{
    pub input_pt: Vec<EcPoint<Fr, CRTInteger<Fr>>>,
    pub output_pt: EcPoint<Fr, CRTInteger<Fr>>,
}

#[derive(Debug, Clone, Copy)]

pub struct LengthCircuitInput{
    pub len: usize,
}

#[derive(Debug, Clone)]

pub struct MulCircuitInput{
    pub input_pt: EcPoint<Fr, CRTInteger<Fr>>,
    pub output_pt: EcPoint<Fr, CRTInteger<Fr>>,
}

pub struct Bn254PrecompileChip<'v, F: PrimeField /*FR:ScalarField*/>{
    //config: Bn254Add_Config,
    range_chip: &'v RangeChip<F>,
    //pairing_chip: &'v PairingChip<'v, F>,
    input_chip: &'v EccChip<'v, F, FpChip<'v, F>>,
}

impl <'a, F: PrimeField> Bn254PrecompileChip<'a, F>{
    
    //initialize/construct the chip
    pub fn construct(
        range_chip: &'a RangeChip<F>,
        //pairing_chip: &'a PairingChip<'a, F>,
        input_chip: &'a EccChip<'a, F, FpChip<'a, F>>,
    ) -> Self{
        Self{
            range_chip,
           // pairing_chip,
            input_chip
        }
    }

    pub fn length_check(
        &self,
        builder: &mut GateThreadBuilder<F>,
        circuit_input: LengthCircuitInput,
    ) {

        let ctx = builder.main(0);

        let x = circuit_input.len as u64;

        let x = F::from(x);

        let x = ctx.load_witness(x);
        
        //check whether length lies in [0, 63)
        let range_check=self.range_chip.range_check(ctx, x, 4);

    }

    pub fn optag_range_check(
        &self,
        builder: &mut GateThreadBuilder<F>,
        circuit_input: OptagCircuitInput,
    ) {
        /*What does range_chip take in as input? A 'Context', an assigned value, and a range for which to check

        In this approach, we initialize a context from the builder, load the witness into the context,
        and then check it using the range chip within the native struct. Converting the input into a witness happens 
        during assignment of the input to the context.
        */
        let ctx = builder.main(0);

        let x = circuit_input.op as u64;

        let x = F::from(x);

        let x = ctx.load_witness(x);
        
        //check whether OpTag lies in [0, 4)
        let range_check=self.range_chip.range_check(ctx, x, 2);

    }



    pub fn check_ec_add(
        &self, 
        builder: &mut GateThreadBuilder<F>,
        input1: EcPoint<F, ProperCrtUint<F>>,
        input2: EcPoint<F, ProperCrtUint<F>>,
    ) -> EcPoint<F, ProperCrtUint<F>> {

        //set up required chips in the main thread

        let ctx = builder.main(0);

        let add_check = self.input_chip.add_unequal(ctx, input1, input2, true);

        add_check
    }

    pub fn check_ec_mul(
        &self, 
        builder: &mut GateThreadBuilder<F>,
        scalar: F,
        input: EcPoint<F, ProperCrtUint<F>>,

    ) {

       let ctx=builder.main(0);

       let scalar_assigned = vec![ctx.load_witness(scalar)];
       
       // let mul_check = self.input_chip.scalar_mult(ctx, input, scalar_assigned), true);

        //mul_check
    }


}













