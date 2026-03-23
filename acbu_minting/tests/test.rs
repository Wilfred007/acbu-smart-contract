use acbu_minting::{MintingContract, MintingContractClient};
use soroban_sdk::{testutils::{Address as _, Events}, Address, Env, String as SorobanString, symbol_short, IntoVal, FromVal, Symbol};
use shared::{MintEvent, BASIS_POINTS};

fn setup_env() -> (Env, Address, Address, Address, Address, Address, i128, MintingContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let reserve_tracker = Address::generate(&env);
    
    let usdc_token = env.register_stellar_asset_contract_v2(admin.clone()).address();
    let acbu_token = env.register_stellar_asset_contract_v2(admin.clone()).address();
    let fee_rate = 300; // 3%

    let contract_id = env.register_contract(None, MintingContract);
    let client = MintingContractClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &oracle,
        &reserve_tracker,
        &acbu_token,
        &usdc_token,
        &fee_rate,
    );

    (env, admin, oracle, reserve_tracker, acbu_token, usdc_token, fee_rate, client)
}

#[test]
fn test_initialize() {
    let (_env, _admin, _oracle, _reserve_tracker, _acbu_token, _usdc_token, fee_rate, client) = setup_env();
    assert_eq!(client.get_fee_rate(), fee_rate);
    assert_eq!(client.is_paused(), false);
}

#[test]
fn test_pause_unpause() {
    let (_env, _admin, _oracle, _reserve_tracker, _acbu_token, _usdc_token, _fee_rate, client) = setup_env();
    assert_eq!(client.is_paused(), false);
    client.pause();
    assert_eq!(client.is_paused(), true);
    client.unpause();
    assert_eq!(client.is_paused(), false);
}

#[test]
fn test_set_fee_rate() {
    let (_env, _admin, _oracle, _reserve_tracker, _acbu_token, _usdc_token, _fee_rate, client) = setup_env();
    let new_fee_rate = 500;
    client.set_fee_rate(&new_fee_rate);
    assert_eq!(client.get_fee_rate(), new_fee_rate);
}

#[test]
fn test_mint_from_usdc() {
    let (env, _admin, _oracle, _reserve_tracker, _acbu_token, usdc_token, fee_rate, client) = setup_env();
    
    let user = Address::generate(&env);
    let recipient = Address::generate(&env);
    let usdc_amount = 1_000_000_000;

    // Mock USDC for user
    let usdc_admin = soroban_sdk::token::StellarAssetClient::new(&env, &usdc_token);
    usdc_admin.mint(&user, &usdc_amount);

    // Call mint_from_usdc
    let acbu_received = client.mint_from_usdc(&user, &usdc_amount, &recipient);

    let expected_fee = (usdc_amount * fee_rate) / BASIS_POINTS;
    let expected_acbu = usdc_amount - expected_fee;
    assert_eq!(acbu_received, expected_acbu);

    // Verify events
    let events = env.events().all();
    let mut found_mint_event = false;
    for event in events.iter() {
        if event.0 != client.address { continue; }
        let topics = event.1;
        if topics.len() > 0 && Symbol::from_val(&env, &topics.get(0).unwrap()) == symbol_short!("mint") {
            let mint_event: MintEvent = event.2.into_val(&env);
            assert_eq!(mint_event.usdc_amount, usdc_amount);
            assert_eq!(mint_event.acbu_amount, acbu_received);
            found_mint_event = true;
        }
    }
    assert!(found_mint_event);
}

#[test]
fn test_mint_from_fiat() {
    let (env, _admin, _oracle, _reserve_tracker, _acbu_token, _usdc_token, fee_rate, client) = setup_env();

    let fintech_partner = Address::generate(&env);
    let recipient = Address::generate(&env);
    let fiat_amount = 1_000_000_000;
    let currency_ngn = SorobanString::from_str(&env, "NGN");
    let fintech_tx_id = SorobanString::from_str(&env, "tx_123");

    // Call mint_from_fiat
    let acbu_received = client.mint_from_fiat(&fintech_partner, &currency_ngn, &fiat_amount, &recipient, &fintech_tx_id);

    let expected_fee = (fiat_amount * fee_rate) / BASIS_POINTS;
    let expected_acbu = fiat_amount - expected_fee;
    assert_eq!(acbu_received, expected_acbu);
}
