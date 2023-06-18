use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, Result};

use ark_std::*;

use halo2_base::halo2_proofs::halo2curves::bn256::G2Affine;
use halo2_ecc::ecc::EccChip;

use halo2_base::{
    gates::{
        builder::{GateThreadBuilder, RangeCircuitBuilder},
        RangeChip,
    },
    utils::fs::gen_srs,
    Context,
    };

use halo2_ecc::fields::FieldChip;
use halo2_ecc::{
    fields::{FpStrategy, PrimeField},
};

use halo2_ecc::bn254::{FpChip, Fp2Chip};

use serde::{Serialize, Deserialize};

use halo2_base::halo2_proofs::halo2curves::bn256::{Fr,G1Affine};

use halo2_proofs::halo2curves::group::cofactor::CofactorCurveAffine;

use halo2_lib_bn254_precompiles::circuits::bn254_table::OpTag;

use itertools::Itertools;

use rand_core::OsRng;

use serde_json::*;

use halo2_proofs::halo2curves::group::Curve;

use halo2_base::halo2_proofs::dev::MockProver;

use core::result::Result as Res;

use halo2_base::gates::RangeInstructions;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CircuitParams {
    strategy: FpStrategy,
    degree: u32,
    num_advice: usize,
    num_lookup_advice: usize,
    num_fixed: usize,
    lookup_bits: usize,
    limb_bits: usize,
    num_limbs: usize,
    batch_size: usize,
}

    fn test_ec_add() {
        let path = "configs/ec_add_circuit.config";
        let params: CircuitParams = serde_json::from_reader(
            File::open(path).unwrap_or_else(|e| panic!("{path} does not exist: {e:?}")),
        )
        .unwrap();
    
        let k = params.degree;
        let points = (0..2).map(|_| G1Affine::random(OsRng)).collect_vec();

        println!("The input points are: {:?}", points);
    
        let output = points.iter().fold(G1Affine::identity(), |a, b| (a + b).to_affine());
        
        println!("The output point is: {:?}", output);

        let mut builder = GateThreadBuilder::<Fr>::mock();


        optag_range_check(&mut builder, params, 1);
        ec_add_test(&mut builder, params, points, output);

        builder.config(k as usize, Some(20));
        let circuit = RangeCircuitBuilder::mock(builder);
        MockProver::run(k, &circuit, vec![]).unwrap();


}

fn test_ec_add_wrong() {
    let path = "configs/ec_add_circuit.config";
    let params: CircuitParams = serde_json::from_reader(
        File::open(path).unwrap_or_else(|e| panic!("{path} does not exist: {e:?}")),
    )
    .unwrap();

    let k = params.degree;
    let points = (0..2).map(|_| G1Affine::random(OsRng)).collect_vec();

    println!("The input points are: {:?}", points);

    let output = G1Affine::random(OsRng);
    
    println!("The output point is: {:?}", output);

    let mut builder = GateThreadBuilder::<Fr>::mock();

    optag_range_check(&mut builder, params, 5);
    
    ec_add_test(&mut builder, params, points, output);

    builder.config(k as usize, Some(20));
    let circuit = RangeCircuitBuilder::mock(builder);
    MockProver::run(k, &circuit, vec![]).unwrap();


}


fn ec_add_test<F:PrimeField>(builder: &mut GateThreadBuilder<F>, params: CircuitParams, _points:Vec<G1Affine>, output: G1Affine){

    let range = RangeChip::<F>::default(params.lookup_bits);
    let fp_chip = FpChip::<F>::new(&range, params.limb_bits, params.num_limbs);
    let ecc_chip = EccChip::new(&fp_chip);

    let ctx = builder.main(0);

    let points =
        _points.iter().map(|pt| ecc_chip.assign_point_unchecked(ctx, *pt)).collect::<Vec<_>>();

    let add_check = ecc_chip.sum::<G1Affine>(ctx, points);

    let x = fp_chip.get_assigned_value(&add_check.x.into());
    let y = fp_chip.get_assigned_value(&add_check.y.into());

    assert_eq!(output.x, x);
    assert_eq!(output.y, y);

    println!("Succesful!!");


}

pub fn optag_range_check<F: PrimeField>(
    builder: &mut GateThreadBuilder<F>,
    params: CircuitParams,
    optag: u64,
) {
    /*What does range_chip take in as input? A 'Context', an assigned value, and a range for which to check

    In this approach, we initialize a context from the builder, load the witness into the context,
    and then check it using the range chip within the native struct. Converting the input into a witness happens 
    during assignment of the input to the context.
    */
    let ctx = builder.main(0);

    let x = optag;

    let x = F::from(x);

    let x = ctx.load_witness(x);
    
    let range = RangeChip::<F>::default(params.lookup_bits);
    //check whether OpTag lies in [0, 4)
    let range_check=range.range_check(ctx, x, 2);

}
/* fn bench_ec_add() -> Res<(), Box<dyn std::error::Error>> {
    let config_path = "configs/bn254/bench_ec_add.config";
    let bench_params_file =
        File::open(config_path).unwrap_or_else(|e| panic!("{config_path} does not exist: {e:?}"));
    fs::create_dir_all("results/bn254").unwrap();

    let results_path = "results/bn254/ec_add_bench.csv";
    let mut fs_results = File::create(results_path).unwrap();
    writeln!(fs_results, "degree,num_advice,num_lookup,num_fixed,lookup_bits,limb_bits,num_limbs,batch_size,proof_time,proof_size,verify_time")?;
    fs::create_dir_all("data").unwrap();

    let bench_params_reader = BufReader::new(bench_params_file);
    for line in bench_params_reader.lines() {
        let bench_params: CircuitParams = serde_json::from_str(line.unwrap().as_str()).unwrap();
        let k = bench_params.degree;
        println!("---------------------- degree = {k} ------------------------------",);
        let mut rng = OsRng;

        let params_time = start_timer!(|| "Params construction");
        let params = gen_srs(k);
        end_timer!(params_time);

        let start0 = start_timer!(|| "Witness generation for empty circuit");

        let circuit = {
            let points_vector=vec![G1Affine::generator(), 2];
            let mut builder = GateThreadBuilder::<Fr>::keygen();
            ec_add_test(&mut builder, params, points, output);


        }

    Ok(())
} */



fn main(){

    test_ec_add();
    test_ec_add_wrong();

//Todo: Transition to chips

}

