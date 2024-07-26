use crate::generated::fiamma::bitvmstaker::MsgCreateStaker as ProtoMsgCreateStaker;
use cosmrs::{tx::Msg, AccountId, ErrorReport, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgCreateStaker {
    pub creator: AccountId,
    pub staker_address: String,
    pub staker_register_id: u64,
}

impl Msg for MsgCreateStaker {
    type Proto = ProtoMsgCreateStaker;
}

impl TryFrom<ProtoMsgCreateStaker> for MsgCreateStaker {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgCreateStaker) -> Result<MsgCreateStaker> {
        MsgCreateStaker::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgCreateStaker> for MsgCreateStaker {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgCreateStaker) -> Result<MsgCreateStaker> {
        Ok(MsgCreateStaker {
            creator: proto.creator.parse()?,
            staker_address: proto.staker_address.parse()?,
            staker_register_id: proto.staker_register_id,
        })
    }
}

impl From<MsgCreateStaker> for ProtoMsgCreateStaker {
    fn from(coin: MsgCreateStaker) -> ProtoMsgCreateStaker {
        ProtoMsgCreateStaker::from(&coin)
    }
}

impl From<&MsgCreateStaker> for ProtoMsgCreateStaker {
    fn from(msg: &MsgCreateStaker) -> ProtoMsgCreateStaker {
        ProtoMsgCreateStaker {
            creator: msg.creator.to_string(),
            staker_address: msg.staker_address.to_string(),
            staker_register_id: msg.staker_register_id,
        }
    }
}
