use crate::domains::entities::user::User; // Import your domain User entity
use crate::infrastructures::repositories::user::UserRepository;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

#[get("/")]
pub async fn get_users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let user_repo = UserRepository::new(pool.get_ref().clone());

    match user_repo.find_all().await {
        Ok(users_data) => {
            let users: Vec<User> = users_data
                .into_iter()
                .map(|user| User {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                })
                .collect();

            HttpResponse::Ok().json(users)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
