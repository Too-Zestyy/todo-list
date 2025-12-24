#[derive(Debug)]
struct Note {
    id: i32,
    name: String,
    description: String
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