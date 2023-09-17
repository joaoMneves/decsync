// TODO: trait syncer Ex: syncthing, drive, filecoin
// methods:
// config<T>(config: ) ex: protos sub
// proto ex: rss_proto
// notify_changes
// fn send_changes(hash, signature, data) -> ex: if untrusted make envelope encription
use crate::util::{differences, signature, Changes};
use crate::{hash, proto, protos, SyncChain};
use async_trait::async_trait;
use openssl::pkey::{PKey, Private};

/// abstraction of the sinchronization method
#[async_trait]
pub trait Syncer {
    type config;
    fn new(config: Self::config) -> Self;
    // fn make_config() -> Self::config;
    fn notify_changes() -> (hash, Vec<signature>);
    fn add_sign(hash: hash, syncchain: &SyncChain<impl Syncer>, sign: signature);
    fn send_changes(hash: hash, data: differences);
    /// observer for local changes
    async fn update(&self, protos: protos, syncchain: &SyncChain<impl Syncer>) {
        let (hash, diff) = syncchain.make_changes(protos);
        let sign = syncchain.verify_changes(hash);
        self.send_changes(hash, diff);
        self.add_sign(hash, syncchain, sign);
    }
}

pub trait observer<T> {
    async fn update(&self, message: T);
}

// impl observer for Syncer {
//  {}
// }
