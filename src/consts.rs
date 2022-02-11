use crate::types;

pub const SOCK_NONBLOCK: types::__socket_type = 2048;
pub const SOCK_CLOEXEC: types::__socket_type = 524288;
pub const SOCK_PACKET: types::__socket_type = 10;
pub const SOCK_DCCP: types::__socket_type = 6;
pub const SOCK_SEQPACKET: types::__socket_type = 5;
pub const SOCK_RDM: types::__socket_type = 4;
pub const SOCK_RAW: types::__socket_type = 3;
pub const SOCK_DGRAM: types::__socket_type = 2;
pub const SOCK_STREAM: types::__socket_type = 1;

pub const VERBOSE5: types::VerboseType = 5;
pub const VERBOSE4: types::VerboseType = 4;
pub const VERBOSE3: types::VerboseType = 3;
pub const VERBOSE2: types::VerboseType = 2;
pub const VERBOSE1: types::VerboseType = 1;
pub const VERBOSE0: types::VerboseType = 0;