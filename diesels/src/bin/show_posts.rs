extern crate diesel;
extern crate diesels;

use self::diesel::prelude::*;
use diesels::establish_connection;
use diesels::models::Post;

fn main() {
    use diesels::schema::posts::dsl::*;

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
