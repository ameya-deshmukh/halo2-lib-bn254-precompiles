//BN254 subcircuit implementation - non-chip approach
#[allow(unused_imports)]

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
    bn254::FpChip,
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


//PLONKish configuration for a 'part' of the larger config
//we have 6 advice columns corresponding to 'Bn254TableRow'

#[derive(Debug, Clone, Copy)]
struct Bn254Add_Config{
    seq_id: Column<Advice>,
    operation: Column<Advice>,
    length: Column<Advice>,
    io: Column<Advice>,
    index: Column<Advice>,
    byte: Column<Advice>,
    always_enabled: Selector, //this is to avoid ConstraintPoisoned
}

