pub mod circuits;


 #[cfg(test)]
mod tests{
    #[test]
    fn add_row_works(){
        use super::circuits::bn254_table::*;

        let new_row=Bn254TableRow::new(1, OpTag::ecAdd, 2, true, 0, 0x01);

         println!("new_row: {:?}", new_row.operation);

    }

    fn optag_range_check_works(){

        use super::circuits::bn254_table::*;

        



    }

}


