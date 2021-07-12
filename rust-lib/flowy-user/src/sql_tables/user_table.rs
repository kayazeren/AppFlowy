use flowy_database::schema::user_table;
use flowy_derive::ProtoBuf;

#[derive(ProtoBuf, Clone, Default, Queryable, Identifiable, Insertable)]
#[table_name = "user_table"]
pub struct User {
    #[pb(index = 1)]
    pub(crate) id: String,

    #[pb(index = 2)]
    pub(crate) name: String,

    #[pb(index = 3)]
    password: String,

    #[pb(index = 4)]
    pub(crate) email: String,
}

impl User {
    pub fn new(id: String, name: String, email: String, password: String) -> Self {
        Self {
            id,
            name,
            email,
            password,
        }
    }
}
