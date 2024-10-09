use crate::generated::fiamma::bitvmstaker::MsgRegisterVk as ProtoMsgRegisterVK;
use cosmrs::{tx::Msg, AccountId, ErrorReport, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgRegisterVK {
    pub creator: AccountId,
    pub vk: Vec<u8>,
}

impl Msg for MsgRegisterVK {
    type Proto = ProtoMsgRegisterVK;
}

impl TryFrom<ProtoMsgRegisterVK> for MsgRegisterVK {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgRegisterVK) -> Result<MsgRegisterVK> {
        MsgRegisterVK::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgRegisterVK> for MsgRegisterVK {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgRegisterVK) -> Result<MsgRegisterVK> {
        Ok(MsgRegisterVK {
            creator: proto.creator.parse()?,
            vk: proto.vk.clone(),
        })
    }
}

impl From<MsgRegisterVK> for ProtoMsgRegisterVK {
    fn from(coin: MsgRegisterVK) -> ProtoMsgRegisterVK {
        ProtoMsgRegisterVK::from(&coin)
    }
}

impl From<&MsgRegisterVK> for ProtoMsgRegisterVK {
    fn from(msg: &MsgRegisterVK) -> ProtoMsgRegisterVK {
        ProtoMsgRegisterVK {
            creator: msg.creator.to_string(),
            vk: msg.vk.clone(),
        }
    }
}
