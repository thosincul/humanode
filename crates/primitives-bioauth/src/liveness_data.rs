//! Plain and opaque Liveness Data.

use core::convert::TryFrom;

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;

/// The data packet required to conduct liveness checks via the FaceTec Server.
#[derive(Debug, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct LivenessData {
    /// The face scan.
    pub face_scan: String,
    /// Audit trail image.
    pub audit_trail_image: String,
    /// Low quality audit trail image.
    pub low_quality_audit_trail_image: String,
}

impl TryFrom<&[u8]> for LivenessData {
    type Error = codec::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::decode(&mut &*value)
    }
}

impl From<&LivenessData> for Vec<u8> {
    fn from(value: &LivenessData) -> Vec<u8> {
        value.encode()
    }
}

/// The opaque encoded form of the [`LivenessData`].
/// Used for signing.
/// Does not guarantee that the underlying bytes indeed represent a valid [`LivenessData`] packet,
/// but allows one to attempt to decode one via [`TryFrom`].
#[derive(Debug, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(transparent))]
pub struct OpaqueLivenessData(pub Vec<u8>);

impl From<Vec<u8>> for OpaqueLivenessData {
    fn from(val: Vec<u8>) -> Self {
        Self(val)
    }
}

impl AsRef<[u8]> for OpaqueLivenessData {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl TryFrom<&OpaqueLivenessData> for LivenessData {
    type Error = codec::Error;

    fn try_from(value: &OpaqueLivenessData) -> Result<Self, Self::Error> {
        Self::try_from(value.0.as_slice())
    }
}

impl From<&LivenessData> for OpaqueLivenessData {
    fn from(val: &LivenessData) -> Self {
        Self(val.into())
    }
}

/// A reference to an opaque encoded form of the [`LivenessData`].
/// Does not guarantee that the underlying bytes indeed represent a valid [`LivenessData`] packet,
/// but allows one to attempt to decode one via [`TryFrom`].
/// For use at encoding and serialization to avoid data copies.
#[derive(Debug, PartialEq, Encode)]
#[cfg_attr(feature = "std", derive(Serialize))]
#[cfg_attr(feature = "std", serde(transparent))]
pub struct OpaqueLivenessDataRef<'a>(pub &'a [u8]);

impl<'a> From<&'a [u8]> for OpaqueLivenessDataRef<'a> {
    fn from(val: &'a [u8]) -> Self {
        Self(val)
    }
}

impl<'a> AsRef<[u8]> for OpaqueLivenessDataRef<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> TryFrom<OpaqueLivenessDataRef<'a>> for LivenessData {
    type Error = codec::Error;

    fn try_from(value: OpaqueLivenessDataRef<'a>) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}