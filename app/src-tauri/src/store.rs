use sled::{Db};
use std::path::PathBuf;
use std::str;

pub struct RefreshTokenStore {
    db: Db,
}

impl RefreshTokenStore {
    pub fn new() -> sled::Result<Self> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("stg");
        path.push("tokens");
        Ok(RefreshTokenStore { db: sled::open(path)? })
    }

    pub fn save(&self, token: &str) -> sled::Result<()> {
        self.db.insert("refresh_token", token.as_bytes())?;
        self.db.flush()
            .map(|_| ())
    }
    pub fn load(&self) -> sled::Result<Option<String>> { 
        if let Some(ivec) = self.db.get("refresh_token")? { 
            Ok(Some(String::from_utf8(ivec.to_vec()).unwrap())) 
        } else { 
            Ok(None) 
        } 
    }
    pub fn delete(&self) -> sled::Result<()> {
        self.db.remove("refresh_token")?;
        self.db.flush()
            .map(|_| ())
    }
}
