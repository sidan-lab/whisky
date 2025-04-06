use cardano_serialization_lib as csl;
use whisky_common::*;

pub fn build_csl_cost_models(network: &Network) -> csl::Costmdls {
    let mut csl_cost_mdls = csl::Costmdls::new();
    let cost_model_list = get_cost_models_from_network(network);
    (0..cost_model_list.len()).for_each(|i| {
        let current_cost_model = &cost_model_list[i];
        if i == 0 {
            csl_cost_mdls.insert(
                &csl::Language::new_plutus_v1(),
                &csl::CostModel::from(
                    current_cost_model
                        .iter()
                        .map(|&i| i as i128)
                        .collect::<Vec<i128>>(),
                ),
            );
        }

        if i == 1 {
            csl_cost_mdls.insert(
                &csl::Language::new_plutus_v2(),
                &csl::CostModel::from(
                    current_cost_model
                        .iter()
                        .map(|&i| i as i128)
                        .collect::<Vec<i128>>(),
                ),
            );
        }

        if i == 2 {
            csl_cost_mdls.insert(
                &csl::Language::new_plutus_v3(),
                &csl::CostModel::from(
                    current_cost_model
                        .iter()
                        .map(|&i| i as i128)
                        .collect::<Vec<i128>>(),
                ),
            );
        }
    });

    csl_cost_mdls
}
