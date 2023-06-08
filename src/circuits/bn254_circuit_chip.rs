
//BN254 subcircuit implementation - chip approach

#[allow(unused_imports)]

use std::marker::PhantomData;




// use ff::{Field as FFField, PrimeField};

//halo2-base imports
use halo2_base::
    {AssignedValue, Context, 
    gates::{
        builder::{GateThreadBuilder, RangeCircuitBuilder},
        GateChip,
        RangeChip,
        GateInstructions
    }, utils::ScalarField,
};

//halo2-ecc imports
use halo2_ecc::{
    bigint::{CRTInteger},
    bn254::{pairing::PairingChip,
            FpChip},
    ecc::{EcPoint, EccChip},
    fields::{FieldChip, PrimeField}
};

//raw halo2 imports 
use halo2_proofs::{
    arithmetic::{CurveAffine, Field, FieldExt},
    dev::MockProver,
    halo2curves::{
        bn256::{Fr, G1Affine, Fq},//F_r is the scalar field and F_q is the base field for the curve
        group:: Curve},
        plonk::{Advice, Column, ConstraintSystem, Error, Expression, Fixed, Selector},
    };

//own imports
use super::bn254_table::*;


struct Bn254PrecompileChip<'v, F: PrimeField, FR:ScalarField>{
    //config: Bn254Add_Config,
    range_chip: &'v RangeChip<FR>,
    pairing_chip: &'v PairingChip<'v, F>,
    input_chip: &'v EccChip<'v, F, FpChip<'v, F>>,
    output_chip: &'v EccChip<'v, F, FpChip<'v, F>>,
}

impl <'a, F: PrimeField, FR: ScalarField> Bn254PrecompileChip<'a, F, FR>{
    pub fn construct(
        range_chip: &'a RangeChip<FR>,
        pairing_chip: &'a PairingChip<'a, F>,
        input_chip: &'a EccChip<'a, F, FpChip<'a, F>>,
        output_chip: &'a EccChip<'a, F, FpChip<'a, F>>,
    ) -> Self{
        Self{
            range_chip,
            pairing_chip,
            input_chip,
            output_chip,
        }
    }
}

//implement OpTag range check, length/index check, and operation checks  






