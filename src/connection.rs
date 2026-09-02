//! [`Connection`] — a [`Session`] extended with an authenticated-user slot.

use std::cell::RefCell;

use crate::errors::Result;
use crate::session::{HttpTransport, Session};
use crate::user::{User, UserSession};

/// A session with an optional authenticated-user slot.
///
/// User state is stored behind a [`RefCell`] so the whole API layer can hold a
/// shared `&Connection` and still record logins background-style, matching how
/// the session itself uses interior mutability for CSRF state.
pub struct Connection {
    session: Session,
    user: RefCell<Option<UserSession>>,
}

impl Connection {
    /// Create a new connection for `base_url`.
    ///
    /// `username`/`password` are accepted for API compatibility with the
    /// Python `Connection`, but login is *not* performed eagerly; call
    /// [`Connection::login`] or let the client log in lazily.
    pub fn new(base_url: &str, _username: Option<&str>, _password: Option<&str>) -> Result<Self> {
        let session = Session::new(base_url)?;
        Ok(Connection {
            session,
            user: RefCell::new(None),
        })
    }

    /// Create a connection backed by a custom [`HttpTransport`].
    ///
    /// Useful for tests: the injected transport receives every request (the
    /// initial CSRF fetch plus all API calls) and can return canned
    /// responses without a real device.
    pub fn with_transport(base_url: &str, transport: Box<dyn HttpTransport>) -> Result<Self> {
        let session = Session::with_transport(base_url, transport)?;
        Ok(Connection {
            session,
            user: RefCell::new(None),
        })
    }

    /// The underlying session.
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// The underlying session (mutable).
    pub fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }

    /// The current authenticated user, if any.
    pub fn user(&self) -> Option<UserSession> {
        self.user.borrow().clone()
    }

    /// Set the current authenticated user.
    pub fn set_user(&self, user: UserSession) {
        *self.user.borrow_mut() = Some(user);
    }

    /// Clear the current authenticated user.
    pub fn clear_user(&self) {
        *self.user.borrow_mut() = None;
    }

    /// Whether the connection is currently authenticated.
    pub fn is_logged_in(&self) -> bool {
        self.user.borrow().is_some()
    }

    /// Convenience: perform a login through the User API group.
    ///
    /// Mirrors the Python pattern:
    ///
    /// ```python
    /// conn = Connection(...)
    /// User(conn).login(username, password)
    /// ```
    pub fn login(&self, username: &str, password: &str) -> crate::Result<bool> {
        let user = User::new(self);
        user.login(username, Some(password), false)
    }
}
