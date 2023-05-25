//BN254 subcircuit implementation

use halo2_base::gates::{GateChip, GateInstructions, RangeChip, RangeInstructions};
use halo2_base::halo2_proofs::halo2curves::group::ff::PrimeField;
use halo2_base::utils::ScalarField;
use halo2_base::{AssignedValue, Context};


use halo2_ecc::ecc::{EcPoint, EccChip};
use halo2_ecc::fields::FieldChip;
use halo2_ecc::ecc::ec_add_unequal;
use halo2_lib_bn254_precompiles::circuits::bn254_table::Bn254TableRow;


use halo2_proofs::{
    arithmetic::Field,
    circuit::{Layouter, Value, SimpleFloorPlanner},  
    plonk::{Advice, Column, ConstraintSystem, Error, Expression, Fixed, Selector},
};

use std::marker::PhantomData;

//helper traits and structs

pub struct ConvertedEcPoint<F: PrimeField, FieldPoint: Clone>(pub EcPoint<F:PrimeField, FieldPoint: Clone>);

impl<F: PrimeField, FieldPoint: Clone> ConvertedEcPoint<F, FieldPoint>{
    pub fn convert_x(&self) -> F {
        self.0.x.clone().into()
    }

    pub fn convert_y(&self) -> F {
        self.0.y.clone().into()
    }
} 


//we take a BN254 row as the table input so no need to define it again 

/*
Verify row is essentially a range check for 
a. whether the row (tag) is within the table range
b. whether the input number of bytes is in the given range for the BN254 curve
c. whether the output number of bytes is in the given range for the BN254 curve
*/
pub fn verify_row<F: ScalarField>(
    ctx: &mut Context<F>,
    row: Bn254TableRow,
    make_public: &mut Vec<AssignedValue<F>>){    

    let tag_check = F::from_str_vartime(&row.tag).expect("deserializing field element shouldn't fail!");
    
    let lookup_bits =
        var("LOOKUP_BITS").unwrap_or_else(|_| panic!("LOOKUP_BITS not set")).parse().unwrap();

    let tag_check=ctx.load_witness(tag_check);

    make_public.push(tag_check);
      
      
    //check that 'tag' of the row to be verified is in [0,4) i.e. 0, 1, 2 or 3

    let range = RangeChip::default(lookup_bits);

    range.range_check(ctx, tag_check, 2);

    //for BN254ADD operation 
    //let gate = GateChip::<F>::default();

    // gate.is_equal(ctx, tag_check, 0);

    //range check for input and output byte size 
    
    

    let converted_input1=ConvertedEcPoint::<F,_>(row.input1);

    let converted_input2 = ConvertedEcPoint::<F,_>(row.input2);

    let converted_output = ConvertedEcPoint::<F,_>(row.output);

    let input1_x_check = converted_input1.convert_x();

    let input1_y_check=converted_input1.convert_y();

    let input2_x_check = converted_input2.convert_x();

    let input2_y_check= converted_input2.convert_y();

    let output_x_check = converted_output.convert_x();

    let output_y_check = converted_output.convert_y();

    
    let input1_x_check=ctx.load_witness(input1_x_check);
    make_public.push(input1_x_check);
    range.range_check(ctx, input1_x_check, 256);


    let input1_y_check=ctx.load_witness(input1_y_check);
    make_public.push(input1_y_check);
    range.range_check(ctx, input1_y_check, 256);

    let input2_x_check=ctx.load_witness(input2_x_check);
    make_public.push(input2_x_check);
    range.range_check(ctx, input2_x_check, 256);

    let input2_y_check = ctx.load_witness(input2_y_check);
    make_public.push(input2_y_check);
    range.range_check(ctx, input2_y_check, 256);

    let output_x_check = ctx.load_witness(output_x_check);
    make_public.push(output_x_check);
    range.range_check(ctx, output_x_check, 256);

    let output_y_check = ctx.load_witness(output_y_check);
    range.range_check(ctx, output_y_check, 256);

    }

pub fn verify_operations<'v, F:ScalarField, FC:FieldChip<F>> (

chip: &FC,
ctx:& mut Context<'v, F>,
row: Bn254TableRow, 
) {

    let P :&EcPoint<'v, F, FC::Point>=&row.input1;

    let Q :&EcPoint<'v, F, FC::Point>=&row.input2;

    let out: &EcPoint<'v, F, FC::Point>=&row.output;

    let add_result = ec_add_unequal(
        chip, ctx, P, Q, is_strict);

    assert_eq!(add_result, out);


}




