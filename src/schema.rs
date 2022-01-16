table! {
    follows (id) {
        id -> Int8,
        user_id -> Int8,
        follower_id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    likes (id) {
        id -> Int8,
        user_id -> Int8,
        tweet_id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    logins (id) {
        id -> Int8,
        token -> Varchar,
        user_id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    tweets (id) {
        id -> Int8,
        tweet -> Text,
        user_id -> Int8,
        like_count -> Int4,
        retweet_count -> Int4,
        comments_count -> Int4,
        retweet_id -> Nullable<Int8>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int8,
        name -> Text,
        username -> Varchar,
        encrypted_password -> Varchar,
        follower_count -> Int4,
        following_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(likes -> tweets (tweet_id));
joinable!(likes -> users (user_id));
joinable!(logins -> users (user_id));
joinable!(tweets -> users (user_id));

allow_tables_to_appear_in_same_query!(
    follows,
    likes,
    logins,
    tweets,
    users,
);
