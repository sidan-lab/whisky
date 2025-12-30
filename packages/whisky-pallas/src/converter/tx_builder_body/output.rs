// use whisky_common::Output;

// use crate::converter::tx_builder_body::convert_value;
// use crate::wrapper::transaction_body::TransactionOutput;

// pub fn convert_outputs(outputs: &Vec<Output>) -> Vec<TransactionOutput> {
//     outputs
//         .into_iter()
//         .map(|output| {
//             TransactionOutput::new(output.address, convert_value(&output.amount), None, None).expect()
//         })
//         .collect()
// }
