-- Your SQL goes here
CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  price INT NOT NULL,
  quantity INT NOT NULL,
  side VARCHAR NOT NULL,
  user_id INT
)