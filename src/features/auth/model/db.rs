use crate::services::neo4rs::get_db;
use neo4rs::query;

use super::types::LoginSchema;

pub async fn find_user_by_email(email: &str) -> Option<LoginSchema> {
    let db = get_db().await.unwrap();

    let mut result = db.execute(
        query("MATCH (p) WHERE (p:Consumer OR p:Shopman) AND p.email = $email RETURN p.id, p.email, p.password, p.user")
            .param("email", email),
    )
    .await.unwrap();

    if let Some(row) = result.next().await.unwrap() {
        let id: String = row.get("p.id").unwrap();
        let email: String = row.get("p.email").unwrap();
        let password: String = row.get("p.password").unwrap();
        let user: String = row.get("p.user").unwrap();

        Some(LoginSchema {
            id,
            email,
            password,
            user,
        })
    } else {
        None
    }
}
