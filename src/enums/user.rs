//! User / authentication related enums.

/// The type of current user (determines which API calls are allowed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentUserType {
    /// Not logged in.
    L0,
    /// Regular user.
    L1,
    /// Super admin.
    L2,
    /// Custom user.
    L3,
    /// Backup admin.
    L4,
    /// Backup admin (variant 2).
    L5,
}

impl CurrentUserType {
    pub fn as_i32(&self) -> i32 {
        match self {
            CurrentUserType::L0 => 0,
            CurrentUserType::L1 => 1,
            CurrentUserType::L2 => 2,
            CurrentUserType::L3 => 3,
            CurrentUserType::L4 => 4,
            CurrentUserType::L5 => 5,
        }
    }

    pub fn from_i32(v: i32) -> CurrentUserType {
        match v {
            1 => CurrentUserType::L1,
            2 => CurrentUserType::L2,
            3 => CurrentUserType::L3,
            4 => CurrentUserType::L4,
            5 => CurrentUserType::L5,
            _ => CurrentUserType::L0,
        }
    }
}

/// The login state of the current user (`LoginStateEnum`).
///
/// Note the Python semantics: `0` means logged in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LoginState {
    /// Logged in.
    LoggedIn = 0,
    /// Logged out.
    LoggedOut = -1,
    /// Login result repeated.
    Repeat = -2,
}

impl LoginState {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(v: i32) -> LoginState {
        match v {
            0 => LoginState::LoggedIn,
            -1 => LoginState::LoggedOut,
            -2 => LoginState::Repeat,
            _ => LoginState::LoggedOut,
        }
    }
}

/// How the password is encoded for the `user/login` request
/// (`PasswordTypeEnum`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PasswordType {
    /// Base64.
    Base64 = 0,
    /// Base64 after password change.
    Base64AfterPasswordChange = 3,
    /// SHA256-derived value.
    Sha256 = 4,
}

impl PasswordType {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(v: i32) -> PasswordType {
        match v {
            3 => PasswordType::Base64AfterPasswordChange,
            4 => PasswordType::Sha256,
            _ => PasswordType::Base64,
        }
    }
}

/// Login error codes (`LoginErrorEnum`).
pub mod error {
    /// Username wrong.
    pub const USERNAME_WRONG: i64 = 108001;
    /// Password wrong.
    pub const PASSWORD_WRONG: i64 = 108002;
    /// Already logged in.
    pub const ALREADY_LOGIN: i64 = 108003;
    /// Username and password both wrong.
    pub const USERNAME_PWD_WRONG: i64 = 108006;
    /// Too many login attempts.
    pub const USERNAME_PWD_OVERRUN: i64 = 108007;
    /// Password must be modified.
    pub const USERNAME_PWD_MODIFY: i64 = 115002;
}
