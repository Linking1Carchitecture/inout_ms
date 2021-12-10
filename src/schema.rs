table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        active -> Bool,
        sign_in_count -> Int8,
    }
}
table!{
    meets (id) {
        id -> Int4,
        m_name -> Varchar,
        m_description -> Text,
        active -> Bool,
    }

}
