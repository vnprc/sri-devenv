#[cfg(not(feature = "with_serde"))]
use alloc::vec::Vec;
#[cfg(not(feature = "with_serde"))]
use binary_sv2::binary_codec_sv2;
use binary_sv2::{Deserialize, Serialize, Str0255, B032};
#[cfg(not(feature = "with_serde"))]
use core::convert::TryInto;

/// Message used by downstream to send result of its hashing work to an upstream.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitSharesStandard {
    /// Channel identification.
    pub channel_id: u32,
    /// Unique sequential identifier of the submit within the channel.
    pub sequence_number: u32,
    /// Identifier of the job as provided by [`NewMiningJob`] or [`NewExtendedMiningJob`] message.
    ///
    /// [`NewMiningJob`]: crate::NewMiningJob
    /// [`NewExtendedMiningJob`]: crate::NewExtendedMiningJob
    pub job_id: u32,
    /// Nonce leading to the hash being submitted.
    pub nonce: u32,
    /// The `nTime` field in the block header. This must be greater than or equal to the
    /// `header_timestamp` field in the latest [`SetNewPrevHash`] message and lower than or equal
    /// to that value plus the number of seconds since the receipt of that message.
    ///
    /// [`SetNewPrevHash`]: crate::SetNewPrevHash
    pub ntime: u32,
    /// Full `nVersion` field.
    pub version: u32,
}

/// Message used by downstream to send result of its hashing work to an upstream.
///
/// The message is the same as [`SubmitShares`], but with an additional field,
/// [`SubmitSharesExtended::extranonce`].
///
/// Only relevant for Extended Channels.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitSharesExtended<'decoder> {
    /// Channel identification.
    pub channel_id: u32,
    /// Unique sequential identifier of the submit within the channel.
    pub sequence_number: u32,
    /// Identifier of the job as provided by [`NewMiningJob`] or [`NewExtendedMiningJob`] message.
    ///
    /// [`NewMiningJob`]: crate::NewMiningJob
    /// [`NewExtendedMiningJob`]: crate::NewExtendedMiningJob
    pub job_id: u32,
    /// Nonce leading to the hash being submitted.
    pub nonce: u32,
    /// The nTime field in the block header. This must be greater than or equal to the
    /// `header_timestamp` field in the latest [`SetNewPrevHash`] message and lower than or equal
    /// to that value plus the number of seconds since the receipt of that message.
    ///
    /// [`SetNewPrevHash`]: crate::SetNewPrevHash
    pub ntime: u32,
    /// Full nVersion field.
    pub version: u32,
    /// Extranonce bytes which need to be added to the coinbase tx to form a fully valid submission
    /// (`full coinbase = coinbase_tx_prefix + extranonce_prefix + extranonce +
    /// coinbase_tx_suffix`).
    ///
    /// The size of the provided extranonce must be equal to the negotiated extranonce size from
    /// channel opening flow.
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub extranonce: B032<'decoder>,
}

/// Message used by upstream to accept [`SubmitSharesStandard`] or [`SubmitSharesExtended`].
///
/// Because it is a common case that shares submission is successful, this response can be provided
/// for multiple [`SubmitShare`] messages aggregated together.
///
/// The upstream doesn’t have to double check that the sequence numbers sent by a downstream are
/// actually increasing. It can use the last one received when sending a response. It is the
/// downstream’s responsibility to keep the sequence numbers correct/useful.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitSharesSuccess {
    /// Channel identifier.
    pub channel_id: u32,
    /// Most recent sequence number with a correct result.
    pub last_sequence_number: u32,
    /// Count of new submits acknowledged within this batch.
    pub new_submits_accepted_count: u32,
    /// Sum of shares acknowledged within this batch.
    pub new_shares_sum: u64,
}

/// Message used by upstream to reject [`SubmitSharesStandard`] or [`SubmitSharesExtended`].
///
/// In case the upstream is not able to immediately validate the submission, the error is sent as
/// soon as the result is known. This delayed validation can occur when a miner gets faster
/// updates about a new `prevhash` than the upstream does.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitSharesError<'decoder> {
    /// Channel identification.
    pub channel_id: u32,
    /// Unique sequential identifier of the submit within the channel.
    pub sequence_number: u32,
    /// Rejection reason.
    ///
    /// Possible error codes:
    ///
    /// - invalid-channel-id
    /// - stale-share
    /// - difficulty-too-low
    /// - invalid-job-id
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub error_code: Str0255<'decoder>,
}

impl<'a> SubmitSharesError<'a> {
    pub fn invalid_channel_error_code() -> &'static str {
        "invalid-channel-id"
    }
    pub fn stale_share_error_code() -> &'static str {
        "stale-share"
    }
    pub fn difficulty_too_low_error_code() -> &'static str {
        "difficulty-too-low"
    }
    pub fn invalid_job_id_error_code() -> &'static str {
        "invalid-job-id"
    }
}
#[cfg(feature = "with_serde")]
use binary_sv2::GetSize;
#[cfg(feature = "with_serde")]
impl GetSize for SubmitSharesStandard {
    fn get_size(&self) -> usize {
        self.channel_id.get_size()
            + self.sequence_number.get_size()
            + self.job_id.get_size()
            + self.nonce.get_size()
            + self.ntime.get_size()
            + self.version.get_size()
    }
}
#[cfg(feature = "with_serde")]
impl<'d> GetSize for SubmitSharesExtended<'d> {
    fn get_size(&self) -> usize {
        self.channel_id.get_size()
            + self.sequence_number.get_size()
            + self.job_id.get_size()
            + self.nonce.get_size()
            + self.ntime.get_size()
            + self.version.get_size()
            + self.extranonce.get_size()
    }
}
#[cfg(feature = "with_serde")]
impl GetSize for SubmitSharesSuccess {
    fn get_size(&self) -> usize {
        self.channel_id.get_size()
            + self.last_sequence_number.get_size()
            + self.new_submits_accepted_count.get_size()
            + self.new_shares_sum.get_size()
    }
}
#[cfg(feature = "with_serde")]
impl<'d> GetSize for SubmitSharesError<'d> {
    fn get_size(&self) -> usize {
        self.channel_id.get_size() + self.sequence_number.get_size() + self.error_code.get_size()
    }
}
#[cfg(feature = "with_serde")]
impl<'a> SubmitSharesError<'a> {
    pub fn into_static(self) -> SubmitSharesError<'static> {
        panic!("This function shouldn't be called by the Message Generator");
    }
    pub fn as_static(&self) -> SubmitSharesError<'static> {
        panic!("This function shouldn't be called by the Message Generator");
    }
}
#[cfg(feature = "with_serde")]
impl<'a> SubmitSharesExtended<'a> {
    pub fn into_static(self) -> SubmitSharesExtended<'static> {
        panic!("This function shouldn't be called by the Message Generator");
    }
    pub fn as_static(&self) -> SubmitSharesExtended<'static> {
        panic!("This function shouldn't be called by the Message Generator");
    }
}
