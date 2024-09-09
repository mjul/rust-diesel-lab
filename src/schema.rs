// @generated automatically by Diesel CLI.

diesel::table! {
    contracts (id) {
        id -> Integer,
        framework_agreement_id -> Integer,
        title -> Text,
        effective_date -> Date,
        seller_id -> Integer,
        buyer_id -> Integer,
    }
}

diesel::table! {
    framework_agreements (id) {
        id -> Integer,
        title -> Text,
        effective_date -> Date,
    }
}

diesel::table! {
    parties (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(contracts -> framework_agreements (framework_agreement_id));

diesel::allow_tables_to_appear_in_same_query!(
    contracts,
    framework_agreements,
    parties,
);
