use sqlx::PgPool;
use w_external_api::GptClient;

pub struct ContextData {
    pub pool: PgPool,
    pub gpt_client: GptClient,
}
