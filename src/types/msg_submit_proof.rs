use crate::generated::fiamma::zkpverify::MsgSubmitProof as ProtoMsgSubmitProof;
use cosmrs::{tx::Msg, AccountId, ErrorReport, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgSubmitProof {
    pub creator: AccountId,
    pub proof_system: String,
    pub proof: Vec<u8>,
    pub public_input: Vec<u8>,
    pub vk: Vec<u8>,
    pub namespace: String,
}

impl Msg for MsgSubmitProof {
    type Proto = ProtoMsgSubmitProof;
}

impl TryFrom<ProtoMsgSubmitProof> for MsgSubmitProof {
    type Error = ErrorReport;

    fn try_from(proto: ProtoMsgSubmitProof) -> Result<MsgSubmitProof> {
        MsgSubmitProof::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgSubmitProof> for MsgSubmitProof {
    type Error = ErrorReport;

    fn try_from(proto: &ProtoMsgSubmitProof) -> Result<MsgSubmitProof> {
        Ok(MsgSubmitProof {
            creator: proto.creator.parse()?,
            proof_system: proto.proof_system.parse()?,
            proof: proto.proof.clone(),
            public_input: proto.public_input.clone(),
            vk: proto.vk.clone(),
            namespace: proto.namespace.parse()?,
        })
    }
}

impl From<MsgSubmitProof> for ProtoMsgSubmitProof {
    fn from(coin: MsgSubmitProof) -> ProtoMsgSubmitProof {
        ProtoMsgSubmitProof::from(&coin)
    }
}

impl From<&MsgSubmitProof> for ProtoMsgSubmitProof {
    fn from(msg: &MsgSubmitProof) -> ProtoMsgSubmitProof {
        ProtoMsgSubmitProof {
            creator: msg.creator.to_string(),
            proof_system: msg.proof_system.to_string(),
            proof: msg.proof.clone(),
            public_input: msg.public_input.clone(),
            vk: msg.vk.clone(),
            namespace: msg.namespace.to_string(),
        }
    }
}
