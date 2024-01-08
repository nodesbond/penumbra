use tendermint::{
    abci::{request::BeginBlock, types::CommitInfo},
    account,
    block::{Header, Height, Round},
    chain,
    validator::Set,
    AppHash, Hash, Time,
};

pub fn begin_block() -> BeginBlock {
    BeginBlock {
        hash: Hash::None,
        header: header(),
        last_commit_info: CommitInfo {
            round: Round::default(),
            votes: vec![],
        },
        byzantine_validators: vec![],
    }
}

fn header() -> Header {
    use tendermint::block::header::Version;
    Header {
        version: Version { block: 0, app: 0 },
        chain_id: chain::Id::try_from("test").unwrap(),
        height: Height::default(),
        time: Time::unix_epoch(),
        last_block_id: None,
        last_commit_hash: None,
        data_hash: None,
        validators_hash: validators().hash(),
        next_validators_hash: validators().hash(),
        consensus_hash: Hash::None,
        app_hash: app_hash(),
        last_results_hash: None,
        evidence_hash: None,
        proposer_address: account::Id::new([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
    }
}

fn validators() -> Set {
    Set::new(vec![], None)
}

fn app_hash() -> AppHash {
    AppHash::try_from(vec![1, 2, 3]).unwrap()
    // AppHash::try_from is infallible, see: https://github.com/informalsystems/tendermint-rs/issues/1243
}

#[cfg(test)]
mod tests {
    #[test]
    fn begin_block_works() {
        let _ = super::begin_block();
        // next, parse this block via a light client
    }
}
