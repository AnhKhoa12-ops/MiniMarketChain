#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol, String,
};

#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub quantity: u32,
}

#[contracttype]
pub enum DataKey {
    Owner,
    Product(u32),
    Revenue,
}

#[contract]
pub struct MiniMarketContract;

#[contractimpl]
impl MiniMarketContract {

    // Khởi tạo owner
    pub fn initialize(env: Env, owner: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Revenue, &0u32);
    }

    // Thêm sản phẩm
    pub fn add_product(
        env: Env,
        caller: Address,
        id: u32,
        name: String,
        price: u32,
        quantity: u32,
    ) {
        caller.require_auth();

        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .unwrap();

        if caller != owner {
            panic!("Only owner can add product");
        }

        let product = Product {
            id,
            name,
            price,
            quantity,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Product(id), &product);
    }

    // Xem sản phẩm
    pub fn get_product(env: Env, id: u32) -> Product {
        env.storage()
            .persistent()
            .get(&DataKey::Product(id))
            .unwrap()
    }

    // Cập nhật số lượng tồn kho
    pub fn update_stock(
        env: Env,
        caller: Address,
        id: u32,
        quantity: u32,
    ) {
        caller.require_auth();

        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .unwrap();

        if caller != owner {
            panic!("Only owner");
        }

        let mut product: Product = env
            .storage()
            .persistent()
            .get(&DataKey::Product(id))
            .unwrap();

        product.quantity = quantity;

        env.storage()
            .persistent()
            .set(&DataKey::Product(id), &product);
    }

    // Bán hàng
    pub fn sell_product(
        env: Env,
        id: u32,
        quantity: u32,
    ) {
        let mut product: Product = env
            .storage()
            .persistent()
            .get(&DataKey::Product(id))
            .unwrap();

        if product.quantity < quantity {
            panic!("Not enough stock");
        }

        product.quantity -= quantity;

        env.storage()
            .persistent()
            .set(&DataKey::Product(id), &product);

        let mut revenue: u32 = env
            .storage()
            .instance()
            .get(&DataKey::Revenue)
            .unwrap();

        revenue += product.price * quantity;

        env.storage()
            .instance()
            .set(&DataKey::Revenue, &revenue);
    }

    // Xem doanh thu
    pub fn get_revenue(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Revenue)
            .unwrap()
    }
}