use crate::generated::fiamma::bitvmstaker::MsgRemoveVk as ProtoMsgRemoveVK;
use cosmrs::{tx::Msg, AccountId, ErrorReport, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgRemoveVK {
    pub creator: AccountId,
    pub vk: Vec<u8>,
}

impl Msg for MsgRemoveVK {
    type Proto = ProtoMsgRemoveVK;
}

impl TryFrom<ProtoMsgRemoveVK> for MsgRemoveVK {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgRemoveVK) -> Result<MsgRemoveVK> {
        MsgRemoveVK::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgRemoveVK> for MsgRemoveVK {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgRemoveVK) -> Result<MsgRemoveVK> {
        Ok(MsgRemoveVK {
            creator: proto.creator.parse()?,
            vk: proto.vk.clone(),
        })
    }
}

impl From<MsgRemoveVK> for ProtoMsgRemoveVK {
    fn from(coin: MsgRemoveVK) -> ProtoMsgRemoveVK {
        ProtoMsgRemoveVK::from(&coin)
    }
}

impl From<&MsgRemoveVK> for ProtoMsgRemoveVK {
    fn from(msg: &MsgRemoveVK) -> ProtoMsgRemoveVK {
        ProtoMsgRemoveVK {
            creator: msg.creator.to_string(),
            vk: msg.vk.clone(),
        }
    }
}
