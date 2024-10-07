use crate::generated::fiamma::bitvmstaker::MsgRemoveVk as ProtoMsgRemoveVk;
use cosmrs::{tx::Msg, AccountId, ErrorReport, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgRemoveVk {
    pub creator: AccountId,
    pub vk: Vec<u8>,
}

impl Msg for MsgRemoveVk {
    type Proto = ProtoMsgRemoveVk;
}

impl TryFrom<ProtoMsgRemoveVk> for MsgRemoveVk {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgRemoveVk) -> Result<MsgRemoveVk> {
        MsgRemoveVk::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgRemoveVk> for MsgRemoveVk {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgRemoveVk) -> Result<MsgRemoveVk> {
        Ok(MsgRemoveVk {
            creator: proto.creator.parse()?,
            vk: proto.vk.clone(),
        })
    }
}

impl From<MsgRemoveVk> for ProtoMsgRemoveVk {
    fn from(coin: MsgRemoveVk) -> ProtoMsgRemoveVk {
        ProtoMsgRemoveVk::from(&coin)
    }
}

impl From<&MsgRemoveVk> for ProtoMsgRemoveVk {
    fn from(msg: &MsgRemoveVk) -> ProtoMsgRemoveVk {
        ProtoMsgRemoveVk {
            creator: msg.creator.to_string(),
            vk: msg.vk.clone(),
        }
    }
}
