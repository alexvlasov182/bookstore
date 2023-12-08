use sqlx::postgres::PgPool;
use sqlx::Row;
use std::error::Error;

#[derive(Debug)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn read(pool: &PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);

    let rows = query.fetch_all(pool).await?;

    let books: Result<Vec<Book>, _> = rows
        .iter()
        .map(|row| {
            Ok::<_, sqlx::Error>(Book {
                title: row.try_get("title")?,
                author: row.try_get("author")?,
                isbn: row.try_get("isbn")?,
            })
        })
        .collect();

    Ok(books?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://dbuser:mysecretpassword@localhost:5432/bookstore";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let books = read(&pool).await?;

    // Now you have a vector of books to work with
    for book in books {
        println!("Title: {}, Author: {}, ISBN: {}", book.title, book.author, book.isbn);
    }

    Ok(())
}
