#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, token};

pub static wage: i128 = 10;

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    blogId
}

#[derive(Clone, PartialEq, Eq)]
#[contracttype]
pub enum DataKey {
    blogToCreator(u32),
    blogToReaders(u32, Address)
}


#[contract]
pub struct MicroPaymentManager {}

#[contractimpl]
impl MicroPaymentManager {

    /// creator can post new blogs, blogId auto-increments
    pub fn addBlog(env: &Env, creator: Address) {
        creator.require_auth();
        let _blogId = env.storage().instance().get::<_, u32>(&StorageKey::blogId).unwrap_or(0) + 1;
        env.storage().instance().set(&DataKey::blogToCreator(_blogId), &creator);

        env.storage().instance().set(&StorageKey::blogId, &_blogId);
    }

    pub fn makeMicroPayment(env: &Env, reader: Address, _blogId: u32) {
        reader.require_auth();

        let mut _creator = env.storage()
            .instance()
            .get(&DataKey::blogToCreator(_blogId))
            .unwrap_or_else(|| panic!("Blog with ID {} does not exist", _blogId));

        let _wage: i128 = env.storage().instance().get(&wage).unwrap();

        MicroPaymentManager::updatePaymentStatus(env, reader, _blogId);
        // existing issue with following line
        // token::Client::new(&env, &reader).transfer(&reader, _creator, &_wage);

    }

    /// to be called after reader pays to creator
    pub fn updatePaymentStatus(env: &Env, addr: Address, index: u32) {
        let value: bool = true;
        env.storage()
            .persistent()
            .set(&DataKey::blogToReaders(index, addr), &value);
    }

    /// get address of creator via blogId
    pub fn getCreator(env: &Env, _blogId: u32) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::blogToCreator(_blogId))
            .unwrap()
    }

    /// check if a reader made the payment for a blog
    pub fn getPaymentStatus(env: &Env, index: u32, addr: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::blogToReaders(index, addr))
            .unwrap_or(false)
    }
    
}