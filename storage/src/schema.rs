table! {
    ingredients (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    receipes (id) {
        id -> Int4,
        name -> Varchar,
        creator -> Nullable<Varchar>,
    }
}

table! {
    receipesingredientlist (id) {
        id -> Int4,
        receipe_id -> Nullable<Int4>,
        ingredient_name -> Nullable<Int4>,
        quantity -> Float8,
    }
}

table! {
    receipessteps (id) {
        id -> Int4,
        receipe_id -> Nullable<Int4>,
        description -> Nullable<Varchar>,
    }
}

joinable!(receipesingredientlist -> ingredients (ingredient_name));
joinable!(receipesingredientlist -> receipes (receipe_id));
joinable!(receipessteps -> receipes (receipe_id));

allow_tables_to_appear_in_same_query!(ingredients, receipes, receipesingredientlist, receipessteps,);
