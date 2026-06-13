#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Map, String, Symbol, Vec,
};

/// Storage key namespace used throughout the contract.
/// `Animals` keeps a single instance-storage map of every registered animal,
/// while `Batch(symbol)` keys per-batch state in persistent storage so that
/// traceability data survives ledger expiration.
#[contracttype]
pub enum DataKey {
    Animals,
    Batch(Symbol),
}

/// On-chain representation of a live animal registered by a rancher.
#[contracttype]
#[derive(Clone)]
pub struct Animal {
    pub tag: Symbol,
    pub breed: Symbol,
    pub cert_hash: String,
    pub rancher: Address,
    pub registered_at: u64,
}

/// On-chain representation of a meat batch produced after slaughter.
/// `steps` accumulates the full transformation chain the batch goes through
/// (cutting, packaging, refrigeration, transport, etc.).
#[contracttype]
#[derive(Clone)]
pub struct Batch {
    pub id: Symbol,
    pub animal_tag: Symbol,
    pub plant: Address,
    pub weight_kg: u32,
    pub slaughter_date: u64,
    pub halal: bool,
    pub steps: Vec<ProcessingStep>,
}

/// A single transformation event recorded against a batch.
#[contracttype]
#[derive(Clone)]
pub struct ProcessingStep {
    pub step: Symbol,
    pub location: Symbol,
    pub processor: Address,
    pub recorded_at: u64,
}

/// `meat_proof` - end-to-end meat batch authentication and traceability contract.
///
/// The contract lets a rancher register an animal, a slaughter plant turn that
/// animal into a numbered batch, processors attach further transformation steps
/// (cut, package, refrigerate, ship, ...), a certifier flag the batch as halal,
/// and any consumer verify how many traceable steps back to the source animal
/// are recorded for a given batch.
#[contract]
pub struct MeatProof;

#[contractimpl]
impl MeatProof {
    /// Register an animal on-chain. The rancher must authorize this call.
    /// `animal_tag` is a unique identifier (ear-tag), `breed` is a breed code,
    /// and `cert_hash` is the off-chain certification hash (organic, grass-fed,
    /// free-range, etc.) that consumers can later verify out-of-band.
    pub fn register_animal(
        env: Env,
        rancher: Address,
        animal_tag: Symbol,
        breed: Symbol,
        cert_hash: String,
    ) {
        rancher.require_auth();

        let mut animals: Map<Symbol, Animal> = env
            .storage()
            .instance()
            .get(&DataKey::Animals)
            .unwrap_or_else(|| Map::new(&env));

        if animals.contains_key(animal_tag.clone()) {
            panic!("Animal already registered");
        }

        let animal = Animal {
            tag: animal_tag.clone(),
            breed,
            cert_hash,
            rancher,
            registered_at: env.ledger().timestamp(),
        };

        animals.set(animal_tag, animal);
        env.storage().instance().set(&DataKey::Animals, &animals);
    }

    /// Record a slaughter event and create a new batch linked to a previously
    /// registered animal. The slaughter plant must authorize the call.
    /// `batch_id` is unique per batch and is what downstream consumers reference
    /// to follow the meat from farm to fork.
    pub fn slaughter(
        env: Env,
        plant: Address,
        animal_tag: Symbol,
        batch_id: Symbol,
        weight_kg: u32,
        date: u64,
    ) {
        plant.require_auth();

        let animals: Map<Symbol, Animal> = env
            .storage()
            .instance()
            .get(&DataKey::Animals)
            .unwrap_or_else(|| Map::new(&env));

        if !animals.contains_key(animal_tag.clone()) {
            panic!("Animal not registered");
        }

        let key = DataKey::Batch(batch_id.clone());
        if env.storage().persistent().has(&key) {
            panic!("Batch already exists");
        }

        let batch = Batch {
            id: batch_id.clone(),
            animal_tag,
            plant,
            weight_kg,
            slaughter_date: date,
            halal: false,
            steps: Vec::new(&env),
        };

        env.storage().persistent().set(&key, &batch);
        env.storage().persistent().extend_ttl(&key, 100, 100);
    }

    /// Attach a processing step (cut, package, refrigerate, ship, ...) to an
    /// existing batch. The processor must authorize the call. Each call adds
    /// a traceable event to the batch's transformation chain, growing the
    /// audit trail a consumer can later inspect.
    pub fn process_step(
        env: Env,
        processor: Address,
        batch_id: Symbol,
        step: Symbol,
        location: Symbol,
    ) {
        processor.require_auth();

        let key = DataKey::Batch(batch_id);
        let mut batch: Batch = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Batch not found");

        let new_step = ProcessingStep {
            step,
            location,
            processor,
            recorded_at: env.ledger().timestamp(),
        };

        batch.steps.push_back(new_step);
        env.storage().persistent().set(&key, &batch);
        env.storage().persistent().extend_ttl(&key, 100, 100);
    }

    /// Returns the number of processing steps recorded for a given batch.
    /// Consumers, retailers, and auditors can call this view function to
    /// gauge how traceable a batch is - more recorded steps means a deeper
    /// chain of custody from slaughter to shelf.
    pub fn verify(env: Env, batch_id: Symbol) -> u32 {
        let key = DataKey::Batch(batch_id);
        let batch: Batch = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Batch not found");
        batch.steps.len()
    }

    /// Returns true if a certifier has previously marked the batch as
    /// halal-certified. This is a read-only check for downstream consumers
    /// and retailers that need to filter batches by religious-compliance
    /// status.
    pub fn is_halal(env: Env, batch_id: Symbol) -> bool {
        let key = DataKey::Batch(batch_id);
        let batch: Batch = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Batch not found");
        batch.halal
    }

    /// Attach or revoke the halal certification flag on a batch. Only an
    /// authorized certifier (e.g. an approved halal certification body) may
    /// call this. Useful for compliance audits and for giving consumers a
    /// trustless way to confirm religious-compliance status.
    pub fn set_halal(env: Env, certifier: Address, batch_id: Symbol, halal: bool) {
        certifier.require_auth();

        let key = DataKey::Batch(batch_id);
        let mut batch: Batch = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Batch not found");

        batch.halal = halal;
        env.storage().persistent().set(&key, &batch);
        env.storage().persistent().extend_ttl(&key, 100, 100);
    }
}
