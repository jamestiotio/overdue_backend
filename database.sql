-- Use this SQL script to set up and initialize the PostgreSQL database

DROP TABLE IF EXISTS leaderboard;
DROP TABLE IF EXISTS material;


CREATE TABLE leaderboard (
    id SERIAL PRIMARY KEY,
    name CHAR(3) NOT NULL,
    gender CHAR(1) NOT NULL,
    email TEXT NOT NULL,
    difficulty INT NOT NULL,
    score INT NOT NULL CHECK (score >= 0 AND score <= 999999),
    materials JSONB,
    creation_date TIMESTAMPTZ NOT NULL DEFAULT (clock_timestamp() AT TIME ZONE 'Asia/Singapore')
);

CREATE TABLE material (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    value INT NOT NULL CHECK (value > 0 AND value <= 1000),
    quantity INT NOT NULL CHECK (quantity >= 0 AND quantity <= 1000)
);


-- Add all valid/verified materials to the material table
INSERT INTO material (name, value, quantity)
    VALUES  ('jigsawAcrylic', 20, 0),
            ('jigsawMetal', 30, 0),
            ('jigsawWood', 20, 0),
            ('cutAcrylic', 50, 0),
            ('cutMetal', 60, 0),
            ('cutWood', 50, 0),
            ('acrylicStrips', 55, 0),
            ('woodStrips', 55, 0),
            ('threeDPrint', 25, 0),
            ('printedPcb', 40, 0);