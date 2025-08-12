pub mod repositories;
pub mod use_cases;
pub mod utils;

use repositories::{
    FilesRepository,
    DirectoriesRepository,
    TagsRepository,
    FileTagsRepository
};
pub use use_cases::{
    TagsUseCase,
    FileTagsUseCase,
    FilesUseCase,
    DirectoriesUseCase
};

const MIGRATIONS: [&str; 2] = [
"CREATE TABLE tags (
  name TEXT NOT NULL PRIMARY KEY,
  description TEXT NOT NULL DEFAULT ''
);",
"CREATE TABLE file_tags (
  file_path TEXT,
  tag_name TEXT NOT NULL,
  FOREIGN KEY (tag_name) REFERENCES tags(name) ON DELETE CASCADE,
  
  UNIQUE (file_path, tag_name)
);"
];
