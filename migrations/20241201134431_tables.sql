-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(55) NOT NULL,
    name VARCHAR(30) NOT NULL,
    password VARCHAR(50) NOT NULL,
    friends JSON NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS room (
    id INT AUTO_INCREMENT,
    members JSON NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS message (
    user_id VARCHAR(55) NOT NULL,
    room_id INT NOT NULL,
    text VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (room_id) REFERENCES room(id)
);