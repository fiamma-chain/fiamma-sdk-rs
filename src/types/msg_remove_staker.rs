use crate::generated::fiamma::bitvmstaker::MsgRemoveStaker as ProtoMsgRemoveStaker;
use cosmrs::{tx::Msg, AccountId, ErrorReport, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgRemoveStaker {
    pub creator: AccountId,
    pub staker_address: String,
}

impl Msg for MsgRemoveStaker {
    type Proto = ProtoMsgRemoveStaker;
}

impl TryFrom<ProtoMsgRemoveStaker> for MsgRemoveStaker {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgRemoveStaker) -> Result<MsgRemoveStaker> {
        MsgRemoveStaker::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgRemoveStaker> for MsgRemoveStaker {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgRemoveStaker) -> Result<MsgRemoveStaker> {
        Ok(MsgRemoveStaker {
            creator: proto.creator.parse()?,
            staker_address: proto.staker_address.parse()?,
        })
    }
}

impl From<MsgRemoveStaker> for ProtoMsgRemoveStaker {
    fn from(coin: MsgRemoveStaker) -> ProtoMsgRemoveStaker {
        ProtoMsgRemoveStaker::from(&coin)
    }
}

impl From<&MsgRemoveStaker> for ProtoMsgRemoveStaker {
    fn from(msg: &MsgRemoveStaker) -> ProtoMsgRemoveStaker {
        ProtoMsgRemoveStaker {
            creator: msg.creator.to_string(),
            staker_address: msg.staker_address.to_string(),
        }
    }
}
