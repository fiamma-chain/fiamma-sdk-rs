use crate::generated::fiamma::zkpverify::MsgSubmitCommunityVerification as ProtoMsgSubmitCommunityVerification;
use cosmrs::{tx::Msg, AccountId, ErrorReport, Result};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgSubmitCommunityVerification {
    pub creator: AccountId,
    pub proof_id: String,
    pub verify_result: bool,
}

impl Msg for MsgSubmitCommunityVerification {
    type Proto = ProtoMsgSubmitCommunityVerification;
}

impl TryFrom<ProtoMsgSubmitCommunityVerification> for MsgSubmitCommunityVerification {
    type Error = ErrorReport;

    fn try_from(
        proto: ProtoMsgSubmitCommunityVerification,
    ) -> Result<MsgSubmitCommunityVerification> {
        MsgSubmitCommunityVerification::try_from(&proto)
    }
}

impl TryFrom<&ProtoMsgSubmitCommunityVerification> for MsgSubmitCommunityVerification {
    type Error = ErrorReport;

    fn try_from(
        proto: &ProtoMsgSubmitCommunityVerification,
    ) -> Result<MsgSubmitCommunityVerification> {
        Ok(MsgSubmitCommunityVerification {
            creator: proto.creator.parse()?,
            proof_id: proto.proof_id.parse()?,
            verify_result: proto.verify_result.clone(),
        })
    }
}

impl From<MsgSubmitCommunityVerification> for ProtoMsgSubmitCommunityVerification {
    fn from(coin: MsgSubmitCommunityVerification) -> ProtoMsgSubmitCommunityVerification {
        ProtoMsgSubmitCommunityVerification::from(&coin)
    }
}

impl From<&MsgSubmitCommunityVerification> for ProtoMsgSubmitCommunityVerification {
    fn from(msg: &MsgSubmitCommunityVerification) -> ProtoMsgSubmitCommunityVerification {
        ProtoMsgSubmitCommunityVerification {
            creator: msg.creator.to_string(),
            proof_id: msg.proof_id.to_string(),
            verify_result: msg.verify_result.clone(),
        }
    }
}
