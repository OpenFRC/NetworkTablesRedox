use std::collections::HashMap;

pub const NT_UNASSIGNED:        char = '\x00';
pub const NT_BOOLEAN:           char = '\x01';
pub const NT_DOUBLE:            char = '\x02';
pub const NT_STRING:            char = '\x04';
pub const NT_RAW:               char = '\x08';
pub const NT_BOOLEAN_ARRAY:     char = '\x10';
pub const NT_DOUBLE_ARRAY:      char = '\x20';
pub const NT_STRING_ARRAY:      char = '\x40';
pub const NT_RPC:               char = '\x80';

pub const NT_NOTIFY_NONE:       char = '\x00';
pub const NT_NOTIFY_IMMEDIATE:  char = '\x01';
pub const NT_NOTIFY_LOCAL:      char = '\x02';
pub const NT_NOTIFY_NEW:        char = '\x04';
pub const NT_NOTIFY_DELETE:     char = '\x08';
pub const NT_NOTIFY_UPDATE:     char = '\x10';
pub const NT_NOTIFY_FLAGS:      char = '\x20';
