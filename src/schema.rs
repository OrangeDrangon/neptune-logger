table! {
    attachments (id) {
        message_id -> Int4,
        id -> Int4,
        filename -> Text,
        binary_data -> Bytea,
    }
}

table! {
    messages (id) {
        id -> Int4,
        author -> Text,
        author_id -> Text,
        channel -> Text,
        channel_id -> Text,
        content -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    attachments,
    messages,
);
