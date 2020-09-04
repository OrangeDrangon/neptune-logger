table! {
    attachments (id) {
        message_id -> Int4,
        id -> Int4,
        filename -> Text,
        binary_data -> Bytea,
        created_at -> Timestamp,
    }
}

table! {
    channels (id) {
        id -> Int4,
        discord_id -> Text,
        name -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    identities (id) {
        user_id -> Int4,
        id -> Int4,
        name -> Text,
        discriminator -> Text,
        nickname -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    messages (id) {
        user_id -> Int4,
        channel_id -> Int4,
        id -> Int4,
        discord_id -> Text,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        discord_id -> Text,
        created_at -> Timestamp,
    }
}

joinable!(attachments -> messages (message_id));
joinable!(identities -> users (user_id));
joinable!(messages -> channels (channel_id));
joinable!(messages -> users (user_id));

allow_tables_to_appear_in_same_query!(
    attachments,
    channels,
    identities,
    messages,
    users,
);
