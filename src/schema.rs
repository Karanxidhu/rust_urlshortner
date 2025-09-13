// @generated automatically by Diesel CLI.

diesel::table! {
    Token (id) {
        id -> Int4,
        userId -> Int4,
        name -> Text,
        mintAddress -> Text,
    }
}

diesel::table! {
    User (id) {
        id -> Int4,
        chatId -> Text,
        defaultWallet -> Int4,
    }
}

diesel::table! {
    Wallet (id) {
        id -> Int4,
        userId -> Int4,
        name -> Text,
        publicKey -> Text,
        privateKey -> Text,
        createdAt -> Timestamp,
    }
}

diesel::table! {
    _prisma_migrations (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 64]
        checksum -> Varchar,
        finished_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamptz>,
        started_at -> Timestamptz,
        applied_steps_count -> Int4,
    }
}

diesel::table! {
    url (id) {
        id -> Int4,
        shorten_url -> Varchar,
        original_url -> Varchar,
        click_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(Token -> User (userId));
diesel::joinable!(Wallet -> User (userId));

diesel::allow_tables_to_appear_in_same_query!(Token, User, Wallet, _prisma_migrations, url,);
