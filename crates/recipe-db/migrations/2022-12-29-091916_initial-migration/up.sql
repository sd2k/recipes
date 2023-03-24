CREATE EXTENSION IF NOT EXISTS "unaccent";

CREATE OR REPLACE FUNCTION slugify(t text) RETURNS text
AS $$
BEGIN
	t := regexp_replace(t, '[Ää]', 'ae', 'g');
	t := regexp_replace(t, '[Öö]', 'oe', 'g');
	t := regexp_replace(t, '[Üü]', 'ue', 'g');
	t := unaccent(t);
	t := lower(t);
	t := regexp_replace(t, '[''"]+', '', 'gi');
	t := regexp_replace(t, '[^a-z0-9\-_]+', '-', 'gi');
	t := regexp_replace(t, '\-+$', '');
	t := regexp_replace(t, '^\-', '');
	RETURN t;
END;
$$ LANGUAGE plpgsql STRICT IMMUTABLE PARALLEL SAFE;

CREATE TABLE recipes (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  name TEXT NOT NULL,
  slug TEXT NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED UNIQUE,
  source TEXT,
  notes TEXT,
  prep_time_minutes INTEGER,
  cooking_time_minutes INTEGER
);

CREATE TABLE recipe_steps (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  recipe_id BIGINT NOT NULL REFERENCES recipes(id),
  step_number INTEGER NOT NULL,
  description TEXT NOT NULL
);

CREATE TABLE measurements (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  name TEXT NOT NULL,
  slug TEXT NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED UNIQUE,
  abbreviation TEXT
);

CREATE UNIQUE INDEX measurements_lower_name_uniq ON measurements (lower(name));

CREATE TABLE ingredients (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  name TEXT NOT NULL,
  slug TEXT NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED UNIQUE,
  default_measurement_id BIGINT NOT NULL REFERENCES measurements(id)
);

CREATE UNIQUE INDEX ingredients_lower_name_uniq ON ingredients (lower(name));

CREATE TABLE recipe_ingredients (
  recipe_id BIGINT NOT NULL REFERENCES recipes(id),
  ingredient_id BIGINT NOT NULL REFERENCES ingredients(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  quantity NUMERIC NOT NULL,
  measurement_id BIGINT REFERENCES measurements(id),
  idx INTEGER NOT NULL,
  notes TEXT,
  PRIMARY KEY (recipe_id, ingredient_id)
);

CREATE TABLE meal_plans (
  id BIGSERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  slug TEXT NOT NULL GENERATED ALWAYS AS (slugify(name)) STORED UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  start_date DATE NOT NULL,
  end_date DATE NOT NULL
);

CREATE TABLE meal_plan_recipes (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  meal_plan_id BIGINT NOT NULL REFERENCES meal_plans(id),
  recipe_id BIGINT NOT NULL REFERENCES recipes(id),
  CONSTRAINT meal_plan_recipes_unique UNIQUE (meal_plan_id, recipe_id)
);
