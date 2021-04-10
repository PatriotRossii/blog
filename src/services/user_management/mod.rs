use sha2::{Digest, Sha256};
use sqlx::{pool::PoolConnection, sqlite::SqliteQueryResult, Pool, Row, Sqlite};

pub struct LoginService {}

impl LoginService {
    fn create_access_token() -> String {
        format!("{}", uuid::Uuid::new_v4())
    }

    pub async fn update_access_token(
        pool: &Pool<Sqlite>,
        user_id: u32,
        access_token: &str,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO access_tokens (user_id, access_token) VALUES (?, ?) ON DUPLICATE KEY UPDATE access_token = VALUES(access_token)"
        ).bind(user_id).bind(access_token).execute(pool).await
    }

    pub async fn get_token_owner(
        pool: &mut PoolConnection<Sqlite>,
        access_token: &str,
    ) -> Option<u32> {
        let token_info = sqlx::query("SELECT user_id FROM access_tokens WHERE access_token = ?")
            .bind(access_token)
            .fetch_one(pool)
            .await;

        if let Ok(row) = token_info {
            row.try_get("user_id").unwrap()
        } else {
            None
        }
    }

    pub async fn try_login(
        pool: &Pool<Sqlite>,
        username: String,
        password: String,
    ) -> Result<String, ()> {
        let user = sqlx::query("SELECT FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(pool);

        if let Ok(row) = user.await {
            let user_hash: &str = row.try_get("hash").unwrap();

            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            let provided_hash = hasher.finalize();

            if user_hash.as_bytes() == &provided_hash[..] {
                let new_access_token = LoginService::create_access_token();
                LoginService::update_access_token(
                    pool,
                    row.try_get("id").unwrap(),
                    &new_access_token,
                )
                .await
                .map_err(|_| ())?;
                return Ok(new_access_token);
            }
        }

        Err(())
    }
}
