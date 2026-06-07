//! Meta Signal contract — privileged `terminal` session lifecycle.
//!
//! Ordinary terminal transport lives in `signal-terminal`. This crate
//! carries the meta-only vocabulary that starts and retires terminal sessions.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_terminal::{TerminalExitStatus, TerminalName};

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct TerminalCommandExecutable(String);

impl TerminalCommandExecutable {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct TerminalCommandArgument(String);

impl TerminalCommandArgument {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalCommand {
    pub executable: TerminalCommandExecutable,
    pub arguments: Vec<TerminalCommandArgument>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct TerminalEnvironmentName(String);

impl TerminalEnvironmentName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct TerminalEnvironmentValue(String);

impl TerminalEnvironmentValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalEnvironmentBinding {
    pub name: TerminalEnvironmentName,
    pub value: TerminalEnvironmentValue,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct TerminalWorkingDirectory(String);

impl TerminalWorkingDirectory {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CreateSession {
    pub name: TerminalName,
    pub command: TerminalCommand,
    pub environment: Vec<TerminalEnvironmentBinding>,
    pub working_directory: Option<TerminalWorkingDirectory>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RetireSession {
    pub name: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SessionCreated {
    pub name: TerminalName,
    pub data_socket_path: signal_persona::WirePath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SessionRetired {
    pub name: TerminalName,
    pub exit_status: Option<TerminalExitStatus>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerTerminalOperationKind {
    CreateSession,
    RetireSession,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OwnerTerminalRequestUnimplemented {
    pub terminal: TerminalName,
    pub operation: OwnerTerminalOperationKind,
    pub reason: OwnerTerminalUnimplementedReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerTerminalUnimplementedReason {
    NotBuiltYet,
    DependencyTrackNotLanded,
}

signal_channel! {
    channel OwnerTerminal {
        operation CreateSession(CreateSession),
        operation RetireSession(RetireSession),
    }
    reply OwnerTerminalReply {
        SessionCreated(SessionCreated),
        SessionRetired(SessionRetired),
        OwnerTerminalRequestUnimplemented(OwnerTerminalRequestUnimplemented),
    }
}

pub type OwnerTerminalRequest = Operation;
pub type OwnerTerminalFrame = Frame;
pub type OwnerTerminalFrameBody = FrameBody;
pub type OwnerTerminalRequestBuilder = RequestBuilder;
pub type ChannelRequest = Operation;
pub type ChannelReply = OwnerTerminalReply;

impl OwnerTerminalRequest {
    pub fn operation_kind(&self) -> OwnerTerminalOperationKind {
        match self {
            Self::CreateSession(_) => OwnerTerminalOperationKind::CreateSession,
            Self::RetireSession(_) => OwnerTerminalOperationKind::RetireSession,
        }
    }
}

impl From<CreateSession> for OwnerTerminalRequest {
    fn from(payload: CreateSession) -> Self {
        Self::CreateSession(payload)
    }
}

impl From<RetireSession> for OwnerTerminalRequest {
    fn from(payload: RetireSession) -> Self {
        Self::RetireSession(payload)
    }
}
