use crate::models::establish_connection;
use crate::schema::users;
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    name: &'a str,
}

impl User {
    pub fn all() -> Vec<User> {
        let connection = &mut establish_connection();
        users::dsl::users
            .limit(30)
            .load::<User>(connection)
            .expect("Error loading users")
    }

    pub fn create(name: &str) -> User {
        use self::users::id;
        let new_user = NewUser { name: name };
        let connection = &mut establish_connection();
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)
            .expect("Error saving new user");

        // 保存したレコードを返す
        users::dsl::users
            .order(id.desc())
            .first::<User>(connection)
            .expect("Error finding users")
    }
}
