-- Your SQL goes here
CREATE TABLE Receipes(
    id SERIAL PRIMARY KEY ,
    name VARCHAR NOT NULL ,
    creator VARCHAR
);

CREATE TABLE ReceipesIngredientList(
    id SERIAL PRIMARY KEY ,
    receipe_id INTEGER REFERENCES Receipes(id),
    ingredient_name INTEGER REFERENCES Ingredients(id),
    quantity FLOAT NOT NULL
);

CREATE TABLE ReceipesSteps(
    id SERIAL PRIMARY KEY ,
    receipe_id INTEGER REFERENCES Receipes(id),
    description VARCHAR
);