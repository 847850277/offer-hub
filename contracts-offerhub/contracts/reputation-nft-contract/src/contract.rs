use crate::access::{
    add_minter as add_minter_impl, check_minter, check_owner, remove_minter as remove_minter_impl,
    transfer_admin as transfer_admin_impl,
};
use crate::events::{emit_minted, emit_transferred, emit_achievement_minted};
use crate::metadata::{get_metadata as get_token_metadata, store_metadata};
use crate::storage::{
    get_admin, get_token_owner, is_minter, save_admin, save_token_owner, token_exists, next_token_id,
};
use crate::{Error, Metadata, TokenId};
use soroban_sdk::{Address, Env, String, Symbol, symbol_short};

pub struct ReputationNFTContract;

impl ReputationNFTContract {
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        save_admin(&env, &admin);
        Ok(())
    }

    pub fn mint(
        env: Env,
        caller: Address,
        to: Address,
        token_id: TokenId,
        name: String,
        description: String,
        uri: String,
    ) -> Result<(), Error> {
        check_minter(&env, &caller)?;
        if token_exists(&env, &token_id) {
            return Err(Error::TokenAlreadyExists);
        }
        save_token_owner(&env, &token_id, &to);
        store_metadata(&env, &token_id, name, description, uri)?;
        emit_minted(&env, &to, &token_id);
        Ok(())
    }

    pub fn mint_achv(env: Env, caller: Address, to: Address, nft_type: Symbol) -> Result<(), Error> {
        check_minter(&env, &caller)?;
        let token_id = next_token_id(&env);
        let (name, description, uri) = match &nft_type {
            s if *s == symbol_short!("tencontr") => (
                String::from_str(&env, "10 Completed Contracts"),
                String::from_str(&env, "Awarded for completing 10 contracts successfully."),
                String::from_str(&env, "ipfs://10-completed-contracts"),
            ),
            s if *s == symbol_short!("5stars5x") => (
                String::from_str(&env, "5 Stars 5 Times"),
                String::from_str(&env, "Awarded for receiving five 5-star reviews."),
                String::from_str(&env, "ipfs://5-stars-5-times"),
            ),
            s if *s == symbol_short!("toprated") => (
                String::from_str(&env, "Top Rated Freelancer"),
                String::from_str(&env, "Awarded for being a top-rated freelancer."),
                String::from_str(&env, "ipfs://top-rated-freelancer"),
            ),
            _ => (
                String::from_str(&env, "Achievement NFT"),
                String::from_str(&env, "Awarded for a special achievement."),
                String::from_str(&env, "ipfs://achievement-generic"),
            ),
        };
        save_token_owner(&env, &token_id, &to);
        store_metadata(&env, &token_id, name, description, uri)?;
        emit_achievement_minted(&env, &to, &nft_type, &token_id);
        Ok(())
    }

    pub fn transfer(env: Env, from: Address, to: Address, token_id: TokenId) -> Result<(), Error> {
        // Check if token exists and get owner
        let owner = get_token_owner(&env, &token_id)?;

        // Validate that from is the owner
        if owner != from {
            return Err(Error::Unauthorized);
        }

        // Check authorization from the owner
        check_owner(&env, &from)?;

        // Update ownership
        save_token_owner(&env, &token_id, &to);

        // Emit transferred event
        emit_transferred(&env, &from, &to, &token_id);

        Ok(())
    }

    pub fn get_owner(env: Env, token_id: TokenId) -> Result<Address, Error> {
        get_token_owner(&env, &token_id)
    }

    pub fn get_metadata(env: Env, token_id: TokenId) -> Result<Metadata, Error> {
        get_token_metadata(&env, &token_id)
    }

    pub fn add_minter(env: Env, caller: Address, minter: Address) -> Result<(), Error> {
        add_minter_impl(&env, &caller, &minter)
    }

    pub fn remove_minter(env: Env, caller: Address, minter: Address) -> Result<(), Error> {
        remove_minter_impl(&env, &caller, &minter)
    }

    pub fn is_minter(env: Env, address: Address) -> Result<bool, Error> {
        Ok(is_minter(&env, &address))
    }

    pub fn get_admin(env: Env) -> Result<Address, Error> {
        Ok(get_admin(&env))
    }

    pub fn transfer_admin(env: Env, caller: Address, new_admin: Address) -> Result<(), Error> {
        transfer_admin_impl(&env, &caller, &new_admin)
    }
}
