use crate::{
    models::{Book, NewBook},
    schema::books,
};
use diesel::prelude::*;
pub struct BookRepo;

impl BookRepo {
    pub fn find_multiple(
        connection: &mut SqliteConnection,
        books_to_get: i64,
    ) -> QueryResult<Vec<Book>> {
        books::table
            .order(books::id.desc())
            .limit(books_to_get)
            .load::<Book>(connection)
    }

    pub fn find_by_id(connection: &mut SqliteConnection, id: i32) -> QueryResult<Book> {
        books::table.find(id).get_result::<Book>(connection)
    }

    pub fn create(connection: &mut SqliteConnection, new_book: NewBook) -> QueryResult<Book> {
        diesel::insert_into(books::table)
            .values(new_book)
            .execute(connection)?;

        let last_id = Self::last_inserted_id(connection)?;
        Self::find_by_id(connection, last_id)
    }

    pub fn save(connection: &mut SqliteConnection, id: i32, book: Book) -> QueryResult<Book> {
        diesel::update(books::table.find(id))
            .set((
                books::name.eq(book.name.to_owned()),
                books::category.eq(book.category.to_owned()),
            ))
            .execute(connection)?;

        Self::find_by_id(connection, id)
    }

    pub fn delete(connection: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(books::table.find(id)).execute(connection)
    }

    fn last_inserted_id(connection: &mut SqliteConnection) -> QueryResult<i32> {
        books::table
            .select(books::id)
            .order(books::id.desc())
            .first(connection)
    }
}
