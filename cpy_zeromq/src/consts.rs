extern crate libc;

#[derive(Copy)]
pub enum SocketType {
    REQ = 3,
    REP = 4,
}

#[derive(Copy)]
pub enum SocketOption {
    TYPE = 16,
}

const HAUSNUMERQ: int = 156384712;

#[derive(PartialEq, Debug, Copy)]
pub enum ErrorCode {
    EINVAL = libc::EINVAL as int,
    EACCES = libc::EACCES as int,
    ECONNREFUSED = libc::ECONNREFUSED as int,
    ECONNRESET = libc::ECONNRESET as int,
    ECONNABORTED = libc::ECONNABORTED as int,
    ENOTCONN = libc::ENOTCONN as int,
    ETIMEOUT = libc::ETIMEOUT as int,
    EPROTONOSUPPORT = HAUSNUMERQ + 2,
    EMSGSIZE = HAUSNUMERQ + 10,
    EFSM = HAUSNUMERQ + 51,
    EIOERROR = HAUSNUMERQ - 1,
}