#[cfg(not(feature = "with_serde"))]
use alloc::vec::Vec;
#[cfg(not(feature = "with_serde"))]
use binary_sv2::binary_codec_sv2::{self, free_vec, CVec};
#[cfg(not(feature = "with_serde"))]
use binary_sv2::Error;
use binary_sv2::{Deserialize, Serialize, B064K};
#[cfg(not(feature = "with_serde"))]
use core::convert::TryInto;

/// Message used by a downstream to submit a successful solution to a previously provided template.
///
/// The downstream is expected to send this message in addition to the `SubmitSolution` message
/// from the Mining Protocol in order to propagate the solution to the Bitcoin network as soon as
/// possible.
///
/// Upon receiving this message, upstream(Template Provider) **must** immediately construct the
/// corresponding full block and attempt to propagate it to the Bitcoin network.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SubmitSolution<'decoder> {
    /// Identifies the template to which this solution corresponds.
    ///
    /// This is acquired from the [`crate::NewTemplate`] message.
    pub template_id: u64,
    /// Version field in the block header.
    ///
    /// Bits not defined by
    /// [BIP320](https://github.com/bitcoin/bips/blob/master/bip-0320.mediawiki) as additional
    /// nonce **must** be the same as they appear in the `NewMiningJob` or `NewExtendedMiningJob`
    /// message, other bits may be set to any value.
    pub version: u32,
    /// nTime field in the block header.
    ///
    /// This **must** be greater than or equal to previously received
    /// [`crate::SetNewPrevHash::header_timestamp`] and lower than or equal to that value plus the
    /// number of seconds since receiving [`crate::SetNewPrevHash`] that message.
    pub header_timestamp: u32,
    /// Nonce field in the header.
    pub header_nonce: u32,
    /// Full serialized coinbase transaction, meeting all the requirements of the `NewMiningJob` or
    /// `NewExtendedMiningJob` message.
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub coinbase_tx: B064K<'decoder>,
}

/// C representation of [`SubmitSolution`].
#[cfg(not(feature = "with_serde"))]
#[repr(C)]
pub struct CSubmitSolution {
    template_id: u64,
    version: u32,
    header_timestamp: u32,
    header_nonce: u32,
    coinbase_tx: CVec,
}

#[cfg(not(feature = "with_serde"))]
impl<'a> CSubmitSolution {
    /// Converts CSubmitSolution(C representation) to SubmitSolution(Rust representation).
    #[cfg(not(feature = "with_serde"))]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_rust_rep_mut(&'a mut self) -> Result<SubmitSolution<'a>, Error> {
        let coinbase_tx: B064K = self.coinbase_tx.as_mut_slice().try_into()?;

        Ok(SubmitSolution {
            template_id: self.template_id,
            version: self.version,
            header_timestamp: self.header_timestamp,
            header_nonce: self.header_nonce,
            coinbase_tx,
        })
    }
}

/// Drops the CSubmitSolution object.
#[no_mangle]
#[cfg(not(feature = "with_serde"))]
pub extern "C" fn free_submit_solution(s: CSubmitSolution) {
    drop(s)
}

#[cfg(not(feature = "with_serde"))]
impl Drop for CSubmitSolution {
    fn drop(&mut self) {
        free_vec(&mut self.coinbase_tx);
    }
}

#[cfg(not(feature = "with_serde"))]
impl<'a> From<SubmitSolution<'a>> for CSubmitSolution {
    fn from(v: SubmitSolution<'a>) -> Self {
        Self {
            template_id: v.template_id,
            version: v.version,
            header_timestamp: v.header_timestamp,
            header_nonce: v.header_nonce,
            coinbase_tx: v.coinbase_tx.into(),
        }
    }
}
#[cfg(feature = "with_serde")]
use binary_sv2::GetSize;
#[cfg(feature = "with_serde")]
impl<'d> GetSize for SubmitSolution<'d> {
    fn get_size(&self) -> usize {
        self.template_id.get_size()
            + self.version.get_size()
            + self.header_timestamp.get_size()
            + self.header_nonce.get_size()
            + self.coinbase_tx.get_size()
    }
}
