use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;
type AllowedUsers = Vec<Principal>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile{
    pub name: String,
    pub descripton: String,
    pub keywords: Vec<String>,
}

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
    static ALLOWED_USERS: RefCell<AllowedUsers> = RefCell::default();
}

#[query(name = "getSelf")]
fn get_self() -> Profile {
    let id = ic_cdk::api::caller();
    PROFILE_STORE.with(|profile_store| {
        profile_store
        .borrow()
        .get(&id)
        .cloned().unwrap_or_default()
    })
}

use ic_cdk::api::caller;

#[update]
fn restricted_action() -> String {
    let user = caller();
    if is_authorized(user) {
        "Dostęp przyznany".to_string()
    } else {
        "Dostęp zabroniony".to_string()
    }
}

fn is_authorized(principal: Principal) -> bool {
    // Sprawdź, czy użytkownik znajduje się na liście uprawnień
    ALLOWED_USERS.with(|users| users.borrow().contains(&principal))
}

#[update]
fn add_user_to_authorised(principal: Principal) {
    ALLOWED_USERS.with(|users| users.borrow_mut().push(principal));
}

#[update]
fn remove_user_from_authorised(principal: Principal) {
    ALLOWED_USERS.with(|users| users.borrow_mut().retain(|&x| x != principal));
}

#[update]
fn add_yourself_to_authorised() {
    add_user_to_authorised(caller());
}

#[update]
fn remove_yourself_from_authorised() {
    remove_user_from_authorised(caller());
}

ic_cdk::export_candid!();