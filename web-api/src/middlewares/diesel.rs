use std::fmt;
use std::fmt::Debug;

#[doc(hidden)]
pub enum ConnectionWrapInner<DB>
where
    DB: Database,
    DB::Connection: Send + Sync + 'static,
{
    Transacting(Transaction<'static, DB>),
    Plain(PoolConnection<DB>),
}

impl<DB> Debug for ConnectionWrapInner<DB>
where
    DB: Database,
    DB::Connection: Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transacting(_) => f.debug_struct("ConnectionWrapInner::Transacting").finish(),
            Self::Plain(_) => f.debug_struct("ConnectionWrapInner::Plain").finish(),
        }
    }
}

impl<DB> Deref for ConnectionWrapInner<DB>
where
    DB: Database,
    DB::Connection: Send + Sync + 'static,
{
    type Target = DB::Connection;

    fn deref(&self) -> &Self::Target {
        match self {
            ConnectionWrapInner::Plain(c) => c,
            ConnectionWrapInner::Transacting(c) => c,
        }
    }
}

impl<DB> DerefMut for ConnectionWrapInner<DB>
where
    DB: Database,
    DB::Connection: Send + Sync + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ConnectionWrapInner::Plain(c) => c,
            ConnectionWrapInner::Transacting(c) => c,
        }
    }
}

#[doc(hidden)]
pub type ConnectionWrap<DB> = Arc<RwLock<ConnectionWrapInner<DB>>>;