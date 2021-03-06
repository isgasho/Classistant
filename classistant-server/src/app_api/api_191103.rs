use std::collections::HashMap;

pub fn register_user_by_nickname(
    db: &crate::db::Database,
    nickname: &str,
    hash_base64: &str,
) -> crate::Result<u64> {
    let mut buf = vec![0u8; 64];
    crate::auth_hash::auth_id_hash(nickname, &mut buf);
    let hash_bytes = base64::decode(hash_base64)?;
    db.register_user_by_nickname(&buf, &hash_bytes)
}

pub fn login_by_auth_id(
    db: &crate::db::Database,
    auth_id_str: &str,
    hash_base64: &str,
) -> crate::Result<u64> {
    let mut buf = vec![0u8; 64];
    crate::auth_hash::auth_id_hash(auth_id_str, &mut buf);
    let hash_bytes = base64::decode(hash_base64)?;
    db.login_by_auth_id(&buf, &hash_bytes)
}
    
pub fn group_create(
    db: &crate::db::Database,
    user_id: u64,
) -> crate::Result<u64> {
    db.group_create(user_id)
}

pub fn group_delete(
    db: &crate::db::Database,
    group_id: u64,
    user_id: u64,
) -> crate::Result<()> {
    db.group_delete(group_id, user_id)
}

pub fn group_transfer_owner(
    db: &crate::db::Database,
    group_id: u64,
    src_user_id: u64,
    dest_user_id: u64,
) -> crate::Result<()> {
    db.group_transfer_owner(group_id, src_user_id, dest_user_id)
}

pub fn data_get_batch(
    db: &crate::db::Database,
    user_id: u64,
    keys: Vec<String>
) -> crate::Result<HashMap<String, (Vec<u8>, Vec<u8>)>> {
    db.data_get_batch(user_id, keys)
}

pub fn data_modify_batch(
    db: &crate::db::Database,
    user_id: u64,
    entries: HashMap<String, (Vec<u8>, Vec<u8>)>,
) -> crate::Result<Vec<String>> {
    db.data_modify_batch(user_id, entries)
}

pub fn data_delete_batch(
    db: &crate::db::Database,
    user_id: u64,
    keys: Vec<String>
) -> crate::Result<()> {
    db.data_delete_batch(user_id, keys)
}

pub fn form_type_create(
    db: &crate::db::Database,
    perm: &str,
    content: &str,
    class: &str,
    extra: &[u8],
) -> crate::Result<u64> { 
    db.form_type_create(perm, content, class, extra)
}

pub fn form_type_get(
    db: &crate::db::Database,
    user_id: u64,
    form_id: u64
) -> crate::Result<(String, String, Vec<u8>)> {
    db.form_type_get(user_id, form_id)
}
