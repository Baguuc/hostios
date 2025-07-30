CREATE TABLE tags (
  name TEXT NOT NULL PRIMARY KEY,
  description TEXT NOT NULL DEFAULT ''
);

CREATE TABLE file_tags (
  file_path TEXT,
  tag_name TEXT NOT NULL,
  FOREIGN KEY (tag_name) REFERENCES tags(name) ON DELETE CASCADE,
  
  UNIQUE (file_path, tag_name)
);
