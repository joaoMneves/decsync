use util::{differences, signature};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod device;
mod syncer;
mod util;

// TOD struct account {
// devices: Vec(device)
// Userid
// keypair
// }
pub struct Account {
    pub devices: device::Device,
    pub user_id: util::Id,
    pub keypair: util::KeyPair,
}

// TODO: move proto, Account to another mod

/// type of data that may be used by one or more apps
pub struct proto {
    /// path items separated by /
    ///protocol can have multiple sub protocols
    /// ex: rss/feed
    ///     rss/saved
    path: String,
    data: Vec<u8>,
}
// type protos = vec<proto>
type hash = Vec<u8>;
type protos = Vec<proto>;

// TODO: struct syncchain {
// local_state: (Account, protos)|| path to dir
// global_state: syncer||
// }
// methods:
// fn new_synchain(local_state, syncer)
// fn make_changes(proto,diff of the changes, device keypair, timestamp, shared encription key)
// -> result<hash>{
//   checks device permission
//   println!("{}", time);
//   hash of (diff, timestamp, device_id) obs: is the name of the directory name
//
// }
// func to check permissions and add new sign
// fn verify_changes(&self, hash, proto) -> signature {sign the hash}
pub struct SyncChain<T: syncer::Syncer> {
    syncer: T,
    account: Account,
    local: protos,
    local_device: device::Device,
}
impl<T: syncer::Syncer> SyncChain<T> {
    pub fn new(
        syncer: T,
        account: Account,
        protos: protos,
        local_device: device::Device,
    ) -> SyncChain<T> {
        Self {
            syncer,
            account,
            local: protos,
            local_device,
        }
    }
    pub fn make_changes(&self, new_protos: protos) -> (hash, differences) {
        // hash of (diff, timestamp, device_id, protos) obs: is the name of the directory name
        unimplemented!()
    }
    pub fn verify_changes(&self, hash: hash) -> signature {
        unimplemented!()
    }
    // TODO: register for syncer observer obs: there will be multiple syncers ex: syncthing, cloud etc
}
// fn subscribe to new proto obs: func to change syncer config or to change device dir
// TODO: syncer will run forever checking changes from global and
// TODO: local protos changes observer
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
