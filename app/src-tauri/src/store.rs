use sled::{Db, IVec};
use std::str;
use std::path::PathBuf;

pub struct RefreshTokenStore {
    db: Db,
}

impl RefreshTokenStore {
    pub fn new() -> sled::Result<Self> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop(); 
        path.push("stg");
        path.push("tokens");
        let db = sled::open(path)?;
        Ok(RefreshTokenStore { db })
    }
    pub fn save(&self, token: &str) -> sled::Result<()> {
        self.db.insert("refresh_token", token.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }
    pub fn load(&self) -> sled::Result<Option<String>> {
        if let Some(ivec) = self.db.get("refresh_token")? {
            Ok(Some(str::from_utf8(&ivec).unwrap().to_string()))
        } else {
            Ok(None)
        }
    }
    pub fn delete(&self) -> sled::Result<()> {
        self.db.remove("refresh_token")?;
        self.db.flush()?;
        Ok(())
    }
}
