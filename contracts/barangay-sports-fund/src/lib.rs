#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAdmin = 3,
    ProposalNotFound = 4,
    AlreadyApproved = 5,
    InsufficientApprovals = 6,
    AlreadyExecuted = 7,
    InvalidAdmins = 8,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Executed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u32,
    pub proposer: Address,
    pub recipient: Address,
    pub amount: i128,
    pub purpose: String,
    pub approvals: Vec<Address>,
    pub status: ProposalStatus,
}

#[contracttype]
pub enum DataKey {
    Admins,
    Token,
    ProposalCount,
    Proposal(u32),
}

#[contract]
pub struct BarangaySportsFundContract;

#[contractimpl]
impl BarangaySportsFundContract {
    /// Initialize the contract with 3 admins and the token address (e.g., XLM).
    pub fn initialize(env: Env, admins: Vec<Address>, token: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admins) {
            return Err(Error::AlreadyInitialized);
        }
        if admins.len() != 3 {
            return Err(Error::InvalidAdmins);
        }
        
        env.storage().instance().set(&DataKey::Admins, &admins);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::ProposalCount, &0u32);
        
        // Extend TTL to keep the contract alive
        env.storage().instance().extend_ttl(1000, 5000);
        
        Ok(())
    }

    /// Donate tokens to the fund.
    pub fn donate(env: Env, donor: Address, amount: i128) -> Result<(), Error> {
        donor.require_auth();
        
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).ok_or(Error::NotInitialized)?;
        let token = soroban_sdk::token::Client::new(&env, &token_addr);
        
        token.transfer(&donor, &env.current_contract_address(), &amount);
        
        Ok(())
    }

    /// Propose a spending withdrawal. Must be an admin.
    pub fn propose_spend(env: Env, admin: Address, recipient: Address, amount: i128, purpose: String) -> Result<u32, Error> {
        admin.require_auth();
        Self::check_admin(&env, &admin)?;

        let mut count: u32 = env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0);
        let proposal_id = count;
        count += 1;
        env.storage().instance().set(&DataKey::ProposalCount, &count);

        let mut approvals = Vec::new(&env);
        approvals.push_back(admin.clone());

        let proposal = Proposal {
            id: proposal_id,
            proposer: admin,
            recipient,
            amount,
            purpose,
            approvals,
            status: ProposalStatus::Pending,
        };

        env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
        env.storage().instance().extend_ttl(1000, 5000);
        
        Ok(proposal_id)
    }

    /// Approve a pending spending proposal. Must be a different admin.
    pub fn approve_spend(env: Env, admin: Address, proposal_id: u32) -> Result<(), Error> {
        admin.require_auth();
        Self::check_admin(&env, &admin)?;

        let mut proposal: Proposal = env.storage().instance().get(&DataKey::Proposal(proposal_id)).ok_or(Error::ProposalNotFound)?;

        if proposal.status == ProposalStatus::Executed {
            return Err(Error::AlreadyExecuted);
        }

        for app in proposal.approvals.iter() {
            if app == admin {
                return Err(Error::AlreadyApproved);
            }
        }

        proposal.approvals.push_back(admin);
        env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
        
        Ok(())
    }

    /// Execute a proposal if it has at least 2 approvals.
    pub fn execute_spend(env: Env, proposal_id: u32) -> Result<(), Error> {
        let mut proposal: Proposal = env.storage().instance().get(&DataKey::Proposal(proposal_id)).ok_or(Error::ProposalNotFound)?;

        if proposal.status == ProposalStatus::Executed {
            return Err(Error::AlreadyExecuted);
        }

        if proposal.approvals.len() < 2 {
            return Err(Error::InsufficientApprovals);
        }

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).ok_or(Error::NotInitialized)?;
        let token = soroban_sdk::token::Client::new(&env, &token_addr);
        
        token.transfer(&env.current_contract_address(), &proposal.recipient, &proposal.amount);

        proposal.status = ProposalStatus::Executed;
        env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
        
        Ok(())
    }

    /// Get the current fund balance.
    pub fn get_balance(env: Env) -> Result<i128, Error> {
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).ok_or(Error::NotInitialized)?;
        let token = soroban_sdk::token::Client::new(&env, &token_addr);
        Ok(token.balance(&env.current_contract_address()))
    }

    /// List all proposals.
    pub fn get_proposals(env: Env) -> Vec<Proposal> {
        let count: u32 = env.storage().instance().get(&DataKey::ProposalCount).unwrap_or(0);
        let mut proposals = Vec::new(&env);
        for i in 0..count {
            if let Some(p) = env.storage().instance().get(&DataKey::Proposal(i)) {
                proposals.push_back(p);
            }
        }
        proposals
    }

    fn check_admin(env: &Env, addr: &Address) -> Result<(), Error> {
        let admins: Vec<Address> = env.storage().instance().get(&DataKey::Admins).ok_or(Error::NotInitialized)?;
        let mut is_admin = false;
        for a in admins.iter() {
            if a == *addr {
                is_admin = true;
                break;
            }
        }
        if is_admin {
            Ok(())
        } else {
            Err(Error::NotAdmin)
        }
    }
}

mod test;
