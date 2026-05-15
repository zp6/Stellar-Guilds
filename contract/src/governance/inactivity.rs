use soroban_sdk::{contracttype, Address, Env, Map, Vec};

const INACTIVITY_THRESHOLD: u64 = 1_000_000;

#[contracttype]
#[derive(Clone, Debug)]
pub struct MemberData {
    pub address: Address,
    pub joined_at: u64,
    pub last_active_at: u64,
}

pub fn is_inactive(env: &Env, members: &Map<Address, MemberData>, member: &Address) -> bool {
    match members.get(member.clone()) {
        Some(data) => env.ledger().sequence().saturating_sub(data.last_active_at) > INACTIVITY_THRESHOLD,
        None => true,
    }
}

pub fn record_activity(env: &Env, members: &mut Map<Address, MemberData>, member: &Address) {
    if let Some(mut data) = members.get(member.clone()) {
        data.last_active_at = env.ledger().sequence();
        members.set(member.clone(), data);
    }
}

pub fn get_inactive_members(env: &Env, members: &Map<Address, MemberData>) -> Vec<Address> {
    let mut inactive = Vec::new(env);
    for (addr, data) in members.iter() {
        if env.ledger().sequence().saturating_sub(data.last_active_at) > INACTIVITY_THRESHOLD {
            inactive.push(addr);
        }
    }
    inactive
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_active_member() {
        let env = Env::default();
        env.ledger().set_sequence(5_000_000);
        let addr = Address::generate(&env);
        let mut members = Map::new(&env);
        members.set(addr.clone(), MemberData { address: addr.clone(), joined_at: 0, last_active_at: 4_500_000 });
        assert!(!is_inactive(&env, &members, &addr));
    }

    #[test]
    fn test_inactive_member() {
        let env = Env::default();
        env.ledger().set_sequence(5_000_000);
        let addr = Address::generate(&env);
        let mut members = Map::new(&env);
        members.set(addr.clone(), MemberData { address: addr.clone(), joined_at: 0, last_active_at: 3_000_000 });
        assert!(is_inactive(&env, &members, &addr));
    }

    #[test]
    fn test_record_activity() {
        let env = Env::default();
        env.ledger().set_sequence(5_000_000);
        let addr = Address::generate(&env);
        let mut members = Map::new(&env);
        members.set(addr.clone(), MemberData { address: addr.clone(), joined_at: 0, last_active_at: 0 });
        record_activity(&env, &mut members, &addr);
        assert_eq!(members.get(addr.clone()).unwrap().last_active_at, 5_000_000);
    }
}
