// @generated automatically by Diesel CLI.

diesel::table! {
    ingredients (id) {
        id -> Int8,
        created_at -> Timestamp,
        name -> Text,
        slug -> Text,
        default_measurement_id -> Int8,
    }
}

diesel::table! {
    meal_plan_recipes (id) {
        id -> Int8,
        created_at -> Timestamp,
        meal_plan_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    meal_plans (id) {
        id -> Int8,
        name -> Text,
        slug -> Text,
        created_at -> Timestamp,
        start_date -> Date,
        end_date -> Date,
    }
}

diesel::table! {
    measurements (id) {
        id -> Int8,
        created_at -> Timestamp,
        name -> Text,
        slug -> Text,
        abbreviation -> Nullable<Text>,
    }
}

diesel::table! {
    recipe_ingredients (recipe_id, ingredient_id) {
        recipe_id -> Int8,
        ingredient_id -> Int8,
        created_at -> Timestamp,
        quantity -> Numeric,
        measurement_id -> Nullable<Int8>,
        idx -> Int4,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    recipe_steps (id) {
        id -> Int8,
        created_at -> Timestamp,
        recipe_id -> Int8,
        step_number -> Int4,
        description -> Text,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int8,
        created_at -> Timestamp,
        name -> Text,
        slug -> Text,
        source -> Nullable<Text>,
        notes -> Nullable<Text>,
        prep_time_minutes -> Nullable<Int4>,
        cooking_time_minutes -> Nullable<Int4>,
    }
}

diesel::joinable!(ingredients -> measurements (default_measurement_id));
diesel::joinable!(meal_plan_recipes -> meal_plans (meal_plan_id));
diesel::joinable!(meal_plan_recipes -> recipes (recipe_id));
diesel::joinable!(recipe_ingredients -> ingredients (ingredient_id));
diesel::joinable!(recipe_ingredients -> measurements (measurement_id));
diesel::joinable!(recipe_ingredients -> recipes (recipe_id));
diesel::joinable!(recipe_steps -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    ingredients,
    meal_plan_recipes,
    meal_plans,
    measurements,
    recipe_ingredients,
    recipe_steps,
    recipes,
);
