CREATE TABLE conversations (
    uuid TEXT,
    title TEXT NOT NULL,
    created_date NUMERIC NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (uuid)
);

CREATE TABLE dialogue_lines (
    id INTEGER,
    conversation_id TEXT,
    "order" INTEGER NOT NULL,
    content TEXT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (conversation_id) REFERENCES conversations (uuid)
);

CREATE TABLE participants (
    id INTEGER,
    conversation_id TEXT,
    "name" TEXT NOT NULL,
    "position" INTEGER NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (conversation_id) REFERENCES conversations (uuid)
);
