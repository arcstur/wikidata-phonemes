use axum::async_trait;
use sqlx::SqlitePool;
use tower_sessions::{
    cookie::SameSite,
    session::{Id, Record},
    Expiry, SessionManagerLayer, SessionStore,
};
use tower_sessions_sqlx_store::SqliteStore;
use tracing::instrument;

const YEAR: time::Duration = time::Duration::days(365);

pub type SessionLayer = SessionManagerLayer<Sessions>;

#[derive(Debug, Clone)]
pub struct Sessions {
    inner: SqliteStore,
}

impl Sessions {
    async fn new(pool: SqlitePool) -> Self {
        let inner = SqliteStore::new(pool);
        inner
            .migrate()
            .await
            .expect("failed to run session migrations");
        Self { inner }
    }

    pub async fn layer(pool: SqlitePool) -> SessionLayer {
        let store = Self::new(pool).await;
        SessionLayer::new(store)
            .with_same_site(SameSite::Lax)
            .with_expiry(Expiry::OnInactivity(YEAR))
    }
}

type SessionResult<T> = Result<T, tower_sessions::session_store::Error>;

#[async_trait]
impl SessionStore for Sessions {
    #[allow(clippy::blocks_in_conditions)]
    #[instrument(skip_all, err, parent = None)]
    async fn create(&self, rec: &mut Record) -> SessionResult<()> {
        self.inner.create(rec).await
    }

    #[allow(clippy::blocks_in_conditions)]
    #[instrument(skip_all, err, parent = None)]
    async fn save(&self, rec: &Record) -> SessionResult<()> {
        self.inner.save(rec).await
    }

    #[allow(clippy::blocks_in_conditions)]
    #[instrument(skip_all, err, parent = None)]
    async fn load(&self, id: &Id) -> SessionResult<Option<Record>> {
        self.inner.load(id).await
    }

    #[allow(clippy::blocks_in_conditions)]
    #[instrument(skip_all, err, parent = None)]
    async fn delete(&self, id: &Id) -> SessionResult<()> {
        self.inner.delete(id).await
    }
}
