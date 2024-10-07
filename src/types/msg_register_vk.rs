use crate::generated::fiamma::bitvmstaker::MsgRegisterVk as ProtoMsgRegisterVk;
use cosmrs::{tx::Msg, AccountId, ErrorReport, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgRegisterVk {
    pub creator: AccountId,
    pub vk: Vec<u8>,
}

impl Msg for MsgRegisterVk {
    type Proto = ProtoMsgRegisterVk;
}

impl TryFrom<ProtoMsgRegisterVk> for MsgRegisterVk {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgRegisterVk) -> Result<MsgRegisterVk> {
        MsgRegisterVk::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgRegisterVk> for MsgRegisterVk {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgRegisterVk) -> Result<MsgRegisterVk> {
        Ok(MsgRegisterVk {
            creator: proto.creator.parse()?,
            vk: proto.vk.clone(),
        })
    }
}

impl From<MsgRegisterVk> for ProtoMsgRegisterVk {
    fn from(coin: MsgRegisterVk) -> ProtoMsgRegisterVk {
        ProtoMsgRegisterVk::from(&coin)
    }
}

impl From<&MsgRegisterVk> for ProtoMsgRegisterVk {
    fn from(msg: &MsgRegisterVk) -> ProtoMsgRegisterVk {
        ProtoMsgRegisterVk {
            creator: msg.creator.to_string(),
            vk: msg.vk.clone(),
        }
    }
}
