// Copyright 2018 Commonwealth Labs, Inc.
// This file is part of Edgeware.

// Edgeware is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Edgeware is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Edgeware.  If not, see <http://www.gnu.org/licenses/>


use substrate_finality_grandpa::AuthorityId as GrandpaId;
use substrate_primitives::{ed25519, sr25519, Pair};
use edgeware_primitives::{AccountId, AuraId};
use edgeware_runtime::{CouncilSeatsConfig, AuraConfig, DemocracyConfig, SystemConfig,
    SessionConfig, StakingConfig, StakerStatus, TimestampConfig, BalancesConfig, TreasuryConfig,
    SudoConfig, ContractConfig, GrandpaConfig, IndicesConfig, Permill, Perbill, SessionKeys,
    IdentityConfig, GovernanceConfig};
pub use edgeware_runtime::GenesisConfig;
use substrate_service;

use substrate_telemetry::TelemetryEndpoints;
use crate::fixtures::*;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "edg";

/// Specialised `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

pub fn edgeware_config() -> ChainSpec {
    match ChainSpec::from_json_file(std::path::PathBuf::from("testnets/v0.2.0/edgeware.json")) {
        Ok(spec) => spec,
        Err(e) => panic!(e),
    }
}

pub fn edgeware_testnet_config_gensis() -> GenesisConfig {
    let initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)> = get_vals();
    let root_key = get_root_key();
    // Add controller accounts to endowed accounts
    let endowed_accounts = initial_authorities.clone().into_iter().map(|elt| elt.1).collect();
    let identity_verifiers = get_identity_verifiers();

    testnet_genesis(
        initial_authorities, // authorities
        root_key,
        Some(endowed_accounts),
        Some(identity_verifiers),
    )
}

/// Edgeware testnet generator
pub fn edgeware_testnet_config() -> Result<ChainSpec, String> {
    let boot_nodes = vec![
        "/ip4/157.230.218.41/tcp/30333/p2p/QmNYiKrVuztYuL42gs5kHLTqvKsmEnE3GvJQ8ewcvwtSVF".to_string(),
        "/ip4/18.223.143.102/tcp/30333/p2p/QmdHoon1jbjeJfTdifknGefGrJHUNYgDDpnJBLLW1Pdt13".to_string(),
        "/ip4/206.189.33.216/tcp/30333/p2p/QmNc7rakvWY1QL6LL9ssTfKTWUhHUfzMvygYdyMLpLQCR7".to_string(),
        "/ip4/157.230.125.18/tcp/30333/p2p/QmTqM3sPbeaE7R2WaveJNg1Ma86dSFPTBHXxYSNjwcii1x".to_string(),
    ];

    Ok(ChainSpec::from_genesis(
        "Edgeware Testnet",
        "edgeware_testnet",
        edgeware_testnet_config_gensis,
        boot_nodes,
        Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
        Some(DEFAULT_PROTOCOL_ID),
        None,
        None
    ))
}

/// Helper function to generate AccountId from seed
pub fn get_account_id_from_seed(seed: &str) -> AccountId {
    sr25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate AuraId from seed
pub fn get_aura_id_from_seed(seed: &str) -> AuraId {
    ed25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate GrandpaId from seed
pub fn get_grandpa_id_from_seed(seed: &str) -> GrandpaId {
    ed25519::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuraId, GrandpaId) {
    (
        get_account_id_from_seed(&format!("{}//stash", seed)),
        get_account_id_from_seed(seed),
        get_aura_id_from_seed(seed),
        get_grandpa_id_from_seed(seed)
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    initial_verifiers: Option<Vec<AccountId>>,
) -> GenesisConfig {
    let initial_verifiers: Vec<AccountId> = initial_verifiers.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed("Alice"),
            get_account_id_from_seed("Bob"),
        ]
    });

    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed("Alice"),
            get_account_id_from_seed("Bob"),
            get_account_id_from_seed("Charlie"),
            get_account_id_from_seed("Dave"),
            get_account_id_from_seed("Eve"),
            get_account_id_from_seed("Ferdie"),
            get_account_id_from_seed("Alice//stash"),
            get_account_id_from_seed("Bob//stash"),
            get_account_id_from_seed("Charlie//stash"),
            get_account_id_from_seed("Dave//stash"),
            get_account_id_from_seed("Eve//stash"),
            get_account_id_from_seed("Ferdie//stash"),
        ]
    });

    const MILLICENTS: u128 = 1_000_000_000;
    const CENTS: u128 = 1_000 * MILLICENTS;    // assume this is worth about a cent.
    const DOLLARS: u128 = 100 * CENTS;

    const SECS_PER_BLOCK: u64 = 6;
    const MINUTES: u64 = 60 / SECS_PER_BLOCK;
    const HOURS: u64 = MINUTES * 60;
    const DAYS: u64 = HOURS * 24;

    const ENDOWMENT: u128 = 10_000_000 * DOLLARS;
    const STASH: u128 = 100 * DOLLARS;

    GenesisConfig {
        system: Some(SystemConfig {
            code: include_bytes!("../../runtime/wasm/target/wasm32-unknown-unknown/release/edgeware_runtime.compact.wasm").to_vec(),    // FIXME change once we have #1252
            _genesis_phantom_data: Default::default(),
            changes_trie_config: Default::default(),
        }),
        balances: Some(BalancesConfig {
            transaction_base_fee: 1 * CENTS,
            transaction_byte_fee: 10 * MILLICENTS,
            balances: endowed_accounts.iter().cloned()
                .map(|k| (k, ENDOWMENT))
                .chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
                .collect(),
            existential_deposit: 1 * DOLLARS,
            transfer_fee: 1 * CENTS,
            creation_fee: 1 * CENTS,
            vesting: vec![],
        }),
        indices: Some(IndicesConfig {
            ids: endowed_accounts.iter().cloned()
                .chain(initial_authorities.iter().map(|x| x.0.clone()))
                .collect::<Vec<_>>(),
        }),
        session: Some(SessionConfig {
            validators: initial_authorities.iter().map(|x| x.1.clone()).collect(),
            keys: initial_authorities.iter().map(|x| (x.1.clone(), SessionKeys(x.2.clone(),x.2.clone()))).collect::<Vec<_>>(),
        }),
        staking: Some(StakingConfig {
            current_era: 0,
            offline_slash: Perbill::from_parts(1_000_000),
            session_reward: Perbill::from_parts(2_065),
            current_session_reward: 0,
            validator_count: 7,
            offline_slash_grace: 4,
            minimum_validator_count: 4,
            stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
            invulnerables: initial_authorities.iter().map(|x| x.1.clone()).collect(),
        }),
        democracy: Some(DemocracyConfig::default()),
        council_seats: Some(CouncilSeatsConfig {
            active_council: vec![],
            candidacy_bond: 10 * DOLLARS,
            voter_bond: 1 * DOLLARS,
            voting_fee: 2 * DOLLARS,
            present_slash_per_voter: 1 * CENTS,
            carry_count: 6,
            presentation_duration: 1 * DAYS,
            approval_voting_period: 2 * DAYS,
            term_duration: 28 * DAYS,
            desired_seats: 0,
            decay_ratio: 0,
            inactive_grace_period: 1,    // one additional vote should go by before an inactive voter can be reaped.
        }),
        timestamp: Some(TimestampConfig {
            minimum_period: SECS_PER_BLOCK / 2, // due to the nature of aura the slots are 2*period
        }),
        treasury: Some(TreasuryConfig {
            proposal_bond: Permill::from_percent(5),
            proposal_bond_minimum: 1 * DOLLARS,
            spend_period: 1 * DAYS,
            burn: Permill::from_percent(50),
        }),
        contract: Some(ContractConfig {
            signed_claim_handicap: 2,
            rent_byte_price: 4,
            rent_deposit_offset: 1000,
            storage_size_offset: 8,
            surcharge_reward: 150,
            tombstone_deposit: 16,
            transaction_base_fee: 1 * CENTS,
            transaction_byte_fee: 10 * MILLICENTS,
            transfer_fee: 1 * CENTS,
            creation_fee: 1 * CENTS,
            contract_fee: 1 * CENTS,
            call_base_fee: 1000,
            create_base_fee: 1000,
            gas_price: 1 * MILLICENTS,
            max_depth: 1024,
            block_gas_limit: 10_000_000,
            current_schedule: Default::default(),
        }),
        sudo: Some(SudoConfig {
            key: root_key,
        }),
        aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
        }),
        grandpa: Some(GrandpaConfig {
            authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
            _genesis_phantom_data: Default::default(),
        }),
        identity: Some(IdentityConfig {
            verifiers: initial_verifiers,
            expiration_length: 7 * DAYS, // 7 days
            registration_bond: 1 * CENTS,
        }),
        governance: Some(GovernanceConfig {
            voting_length: 7 * DAYS, // 7 days
            proposal_creation_bond: 1 * CENTS,
        }),
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
        ],
        get_account_id_from_seed("Alice"),
        None,
        None,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        development_config_genesis,
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        None)
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
            get_authority_keys_from_seed("Bob"),
        ],
        get_account_id_from_seed("Alice"),
        None,
        None,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        local_testnet_genesis,
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        None)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use service_test;
    use crate::Factory;

    fn local_testnet_genesis_instant() -> GenesisConfig {
        let mut genesis = local_testnet_genesis();
        genesis.timestamp = Some(TimestampConfig { minimum_period: 1 });
        genesis
    }

    fn local_testnet_genesis_instant_single() -> GenesisConfig {
        let mut genesis = testnet_genesis(
            vec![
                get_authority_keys_from_seed("Alice"),
            ],
            get_account_id_from_seed("Alice"),
            None,
            None,
        );
        genesis.timestamp = Some(TimestampConfig { minimum_period: 1 });
        genesis
    }

    /// Local testnet config (single validator - Alice)
    pub fn integration_test_config_with_single_authority() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            local_testnet_genesis_instant_single,
            vec![],
            None,
            None,
            None,
            None,
        )
    }

    /// Local testnet config (multivalidator Alice + Bob)
    pub fn integration_test_config_with_two_authorities() -> ChainSpec {
        ChainSpec::from_genesis("Integration Test", "test", local_testnet_genesis_instant, vec![], None, None, None, None)
    }

    #[test]
    #[ignore]
    fn test_connectivity() {
        service_test::connectivity::<Factory>(integration_test_config_with_two_authorities());
    }
}
