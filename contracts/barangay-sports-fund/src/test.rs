#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Events}, Address, Env, IntoVal, String, vec};

#[test]
fn test_fund_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(BarangaySportsFundContract, ());
    let client = BarangaySportsFundContractClient::new(&env, &contract_id);

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let admin3 = Address::generate(&env);
    let admins = vec![&env, admin1.clone(), admin2.clone(), admin3.clone()];

    // Register a token for testing
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    let token = soroban_sdk::token::Client::new(&env, &token_id);
    let token_admin_client = soroban_sdk::token::StellarAssetClient::new(&env, &token_id);

    client.initialize(&admins, &token_id);

    let donor = Address::generate(&env);
    token_admin_client.mint(&donor, &1000);

    // Test Donation
    client.donate(&donor, &500);
    assert_eq!(client.get_balance(), 500);
    assert_eq!(token.balance(&contract_id), 500);

    // Test Proposal
    let recipient = Address::generate(&env);
    let purpose = String::from_str(&env, "Basketballs");
    let proposal_id = client.propose_spend(&admin1, &recipient, &200, &purpose);
    assert_eq!(proposal_id, 0);

    // Check proposals
    let proposals = client.get_proposals();
    assert_eq!(proposals.len(), 1);
    assert_eq!(proposals.get(0).unwrap().status, ProposalStatus::Pending);
    assert_eq!(proposals.get(0).unwrap().approvals.len(), 1);

    // Test Approval (Admin 2)
    client.approve_spend(&admin2, &0);
    let proposals = client.get_proposals();
    assert_eq!(proposals.get(0).unwrap().approvals.len(), 2);

    // Test Execution
    client.execute_spend(&0);
    assert_eq!(client.get_balance(), 300);
    assert_eq!(token.balance(&recipient), 200);

    let proposals = client.get_proposals();
    assert_eq!(proposals.get(0).unwrap().status, ProposalStatus::Executed);
}

#[test]
fn test_unauthorized_spend() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(BarangaySportsFundContract, ());
    let client = BarangaySportsFundContractClient::new(&env, &contract_id);

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let admin3 = Address::generate(&env);
    let admins = vec![&env, admin1.clone(), admin2.clone(), admin3.clone()];

    let token_id = Address::generate(&env); // Dummy token address
    client.initialize(&admins, &token_id);

    let non_admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let purpose = String::from_str(&env, "Theft");

    // Proposing by non-admin should fail (it will actually fail in check_admin)
    let res = client.try_propose_spend(&non_admin, &recipient, &100, &purpose);
    assert!(res.is_err());
}
