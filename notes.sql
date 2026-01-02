-- sqlite3
-- .read ./notes.sql

create table if not exists ingredients (
  id integer primary key,
  name text not null
) strict;

create table if not exists units (
  id integer primary key,
  name text not null
) strict;

create table if not exists recipes (
  id integer primary key,
  name text not null,
  description text not null
) strict;

create table if not exists inventory (
  id integer primary key,
  amount integer not null,
  unit integer not null references units,
  ingredient integer not null references ingredients
) strict;

create table if not exists recipe_ingredients (
  ingredient integer not null references ingredients,
  recipe integer not null references recipes,
  amount integer not null,
  unit integer not null references units,
  PRIMARY KEY (ingredient, recipe)
) strict;


-- populate
insert into ingredients (name)
values 
  ('milk'),
  ('wheaties');

insert into units (name)
values ('cup');

insert into inventory (amount, unit, ingredient)
values (
    4, 
    (select id from units where name = 'cup'), 
    (select id from ingredients where name = 'milk')
  ),
  (
    12,
    (select id from units where name = 'cup'),
    (select id from ingredients where name = 'wheaties')
  );

insert into recipes (name, description)
values ('bowl of cereal', 'A simple bowl of cereal');

insert into recipe_ingredients (ingredient, recipe, amount, unit)
values (
  (select id from ingredients where name = 'milk'),
  (select id from recipes where name = 'bowl of cereal'),
  1,
  (select id from units where name = 'cup')
),
(
  (select id from ingredients where name = 'wheaties'),
  (select id from recipes where name = 'bowl of cereal'),
  1,
  (select id from units where name = 'cup')
);

-- select full inventory
select 
  inventory.id,
  ingredients.name,
  inventory.amount,
  units.name
from inventory
join ingredients on ingredients.id = inventory.ingredient
join units on units.id = inventory.unit;

-- select ingredients bowl of cereal recipe
select
  recipes.name,
  ingredients.name,
  recipe_ingredients.amount,
  units.name
from recipe_ingredients
join ingredients on ingredients.id = recipe_ingredients.ingredient
join recipes on recipes.id = recipe_ingredients.recipe
join units on units.id = recipe_ingredients.unit
where recipes.name = 'bowl of cereal';

drop table recipe_ingredients;
drop table inventory;
drop table recipes;
drop table units;
drop table ingredients;

