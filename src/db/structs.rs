#[derive(Debug)]
pub struct Note {
    pub id: u32,
    pub name: String,
    pub description: String,

    pub date_created_utc: String,
    pub last_updated_utc: String,
}

#[derive(Debug)]
struct CategoryTag {
    id: i32,
    name: String,
    description: String
}

#[derive(Debug)]
struct NoteTags {
    note_id: i32,
    category_tag_id: i32
}
