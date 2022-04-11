use hex_literal::hex;
use node_kitties_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature,
	SubstrateKittiesConfig, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::UncheckedInto;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

type Balance = u128;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

pub fn kitty_testnest_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let initial_authorities: Vec<(AuraId, GrandpaId)> = vec![
		(
			hex!("c8d89da713932679a40c5daaf5a589e82a96f1ac9e19c931c2f285003cdb1f0d")
				.unchecked_into(),
			hex!("d5cd2a4406d231b02ee02fcdbb47812f1f35b237849afbd0b67c09b77c82d684")
				.unchecked_into(),
		),
		(
			hex!("9413c544aabd11206fa8d7744ceecd3a335495e612d2429ce68cc4f668fd1609")
				.unchecked_into(),
			hex!("6a621cd177b805482a542252bab82278c98873f8a77bf7dcab839cee6e0964e0")
				.unchecked_into(),
		),
		(
			hex!("d4b6ed5487829b47208e85ddac7e193132c6c0bf8b649e513fae181a5a038e7e")
				.unchecked_into(),
			hex!("81bbe8ca4fdbaa010c41b0ec8199f6648e86146294489658248e82c184c099a9")
				.unchecked_into(),
		),
	];

	let pre_funded_account: Vec<(AccountId, Balance)> = vec![
		(
			hex!("c8d89da713932679a40c5daaf5a589e82a96f1ac9e19c931c2f285003cdb1f0d").into(),
			100_000_000_000_000_000,
		),
		(
			hex!("9413c544aabd11206fa8d7744ceecd3a335495e612d2429ce68cc4f668fd1609").into(),
			200_000_000_000_000_000,
		),
		(
			hex!("d4b6ed5487829b47208e85ddac7e193132c6c0bf8b649e513fae181a5a038e7e").into(),
			300_000_000_000_000_000,
		),
		(
			hex!("f8460135a61d93b25b1f4b35112795b0305470da961c26b8349d08ac8ca0227f").into(),
			300_000_000_000_000_000,
		),
	];

	Ok(ChainSpec::from_genesis(
		// Name
		"Kitty Testnest",
		// ID
		"kit",
		ChainType::Live,
		move || {
			kitty_testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				initial_authorities.clone(),
				// Sudo account
				hex!("f8460135a61d93b25b1f4b35112795b0305470da961c26b8349d08ac8ca0227f").into(),
				// Pre-funded accounts
				pre_funded_account.clone(),
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(
			serde_json::from_str(
				"{\"tokenDecimals\": 18, \"tokenSymbol\": \"KIT\", \"SS58Prefix\": 48}",
			)
			.expect("Provided valid json map"),
		),
		// Extensions
		None,
	))
}

fn kitty_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k.0, k.1)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		substrate_kitties: SubstrateKittiesConfig { kitties: vec![] },
	}
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		substrate_kitties: SubstrateKittiesConfig { kitties: vec![] },
	}
}
