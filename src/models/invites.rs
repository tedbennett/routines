use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{database::Database, error::ApiResult};

#[derive(sqlx::Type, PartialEq)]
pub enum InviteStatus {
    Sent,
    Accepted,
    Revoked,
}

#[derive(FromRow)]
pub struct Invite {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub status: InviteStatus,
    pub created_at: OffsetDateTime,
}

impl From<String> for InviteStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "sent" => Self::Sent,
            "accepted" => Self::Accepted,
            "revoked" => Self::Revoked,
            _ => panic!("Failed to parse InviteStatus: {value}"),
        }
    }
}

pub trait InviteDataLayer {
    async fn get_invite<'a>(&'a self, id: &'a Uuid) -> ApiResult<Option<Invite>>;
    async fn create_invite<'a>(&'a self, sender: &'a Uuid) -> ApiResult<Uuid>;
    async fn update_invite<'a>(&'a self, id: &'a Uuid, status: InviteStatus) -> ApiResult<()>;
}

impl InviteDataLayer for Database {
    async fn get_invite<'a>(&'a self, id: &'a Uuid) -> ApiResult<Option<Invite>> {
        let invite = sqlx::query_as::<_, Invite>(
            r#"SELECT id, sender_id, status, created_at FROM invite WHERE id = ?"#,
        )
        .bind(&id)
        .fetch_optional(&self.db)
        .await?;

        Ok(invite)
    }

    async fn create_invite<'a>(&'a self, sender: &'a Uuid) -> ApiResult<Uuid> {
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        sqlx::query(
            r#"INSERT INTO invite (id, sender_id, status, created_at) VALUES ($1, $2, $3, $4) "#,
        )
        .bind(id)
        .bind(sender)
        .bind(InviteStatus::Sent)
        .bind(now)
        .execute(&self.db)
        .await?;
        Ok(id)
    }

    async fn update_invite<'a>(&'a self, id: &'a Uuid, status: InviteStatus) -> ApiResult<()> {
        sqlx::query(r#"UPDATE invite SET status = ? WHERE id = ?"#)
            .bind(status)
            .bind(id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
