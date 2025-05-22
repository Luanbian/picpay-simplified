use crate::services::neo4rs::get_db;
use neo4rs::query;

use super::types::LoginSchema;

pub async fn find_user_by_email(email: &str) -> Option<LoginSchema> {
    let db = get_db().await.unwrap();

    let mut result = db.execute(
        query("MATCH (p) WHERE (p:Consumer OR p:Shopman) AND p.email = $email RETURN p.email, p.password")
            .param("email", email),
    )
    .await.unwrap();

    if let Some(row) = result.next().await.unwrap() {
        let email: String = row.get("p.email").unwrap();
        let password: String = row.get("p.password").unwrap();

        Some(LoginSchema { email, password })
    } else {
        None
    }
}
