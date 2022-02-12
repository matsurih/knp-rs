use crate::juman::structs::C2RustUnnamed;
use crate::juman::types::_ExitCode;

pub const OtherError: _ExitCode = 11;
pub const UnknownId: _ExitCode = 10;
pub const SyntaxError: _ExitCode = 9;
pub const ProgramError: _ExitCode = 8;
pub const ConfigError: _ExitCode = 7;
pub const ConnError: _ExitCode = 6;
pub const DicError: _ExitCode = 5;
pub const GramError: _ExitCode = 4;
pub const AllocateError: _ExitCode = 3;
pub const OpenError: _ExitCode = 2;
pub const SystemError: _ExitCode = 1;
pub const NormalExit: _ExitCode = 0;

pub const _ISalnum: C2RustUnnamed = 8 as C2RustUnnamed;
pub const _ISpunct: C2RustUnnamed = 4 as C2RustUnnamed;
pub const _IScntrl: C2RustUnnamed = 2 as C2RustUnnamed;
pub const _ISblank: C2RustUnnamed = 1 as C2RustUnnamed;
pub const _ISgraph: C2RustUnnamed = 32768 as C2RustUnnamed;
pub const _ISprint: C2RustUnnamed = 16384 as C2RustUnnamed;
pub const _ISspace: C2RustUnnamed = 8192 as C2RustUnnamed;
pub const _ISxdigit: C2RustUnnamed = 4096 as C2RustUnnamed;
pub const _ISdigit: C2RustUnnamed = 2048 as C2RustUnnamed;
pub const _ISalpha: C2RustUnnamed = 1024 as C2RustUnnamed;
pub const _ISlower: C2RustUnnamed = 512 as C2RustUnnamed;
pub const _ISupper: C2RustUnnamed = 256 as C2RustUnnamed;