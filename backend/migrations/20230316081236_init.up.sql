-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS feedbacks (
        id CHAR(36) PRIMARY KEY NOT NULL,
        rating INTEGER NOT NULL,
        comment VARCHAR(255) NOT NULL UNIQUE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
    );