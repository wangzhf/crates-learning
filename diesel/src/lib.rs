#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::models::NewPost;
use crate::models::Post;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub mod models;
pub mod schema;

// 建立连接
pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post ")
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::models::Post;
    use crate::schema::posts::dsl::*;

    #[test]
    fn test_query() {
        let connection = establish_connection();
        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .load::<Post>(&connection)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("------------\n");
            println!("{}", post.body);
        }
    }
}
