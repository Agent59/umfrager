CREATE TABLE usrpoints (
  id TEXT NOT NULL PRIMARY KEY,
  points INTEGER NOT NULL,
  gamename_id TEXT NOT NULL REFERENCES gamename(name),
  usr_id TEXT NOT NULL REFERENCES usr(id)
)
