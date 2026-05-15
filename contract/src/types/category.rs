use soroban_sdk::{contracttype, BytesN, Address, Bytes};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum BountyCategory {
    Development,
    Design,
    Documentation,
    Research,
    Other,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Bounty {
    pub id: BytesN<32>,
    pub title: Bytes,
    pub category: BountyCategory,
    pub reward: i128,
    pub creator: Address,
    pub created_at: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_category_enum() {
        let env = soroban_sdk::Env::default();
        let cat = BountyCategory::Development;
        assert_eq!(cat, BountyCategory::Development);
        assert_ne!(cat, BountyCategory::Design);
    }

    #[test]
    fn test_bounty_with_category() {
        let env = soroban_sdk::Env::default();
        let bounty = Bounty {
            id: BytesN::from_array(&env, &[0u8; 32]),
            title: Bytes::from_slice(&env, b"Test"),
            category: BountyCategory::Development,
            reward: 1000,
            creator: Address::generate(&env),
            created_at: 0,
        };
        assert_eq!(bounty.category, BountyCategory::Development);
    }
}
