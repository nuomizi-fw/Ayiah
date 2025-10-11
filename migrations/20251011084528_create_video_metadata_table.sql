-- Add migration script here
-- Video metadata table (for movies and TV shows)
CREATE TABLE IF NOT EXISTS video_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    media_item_id INTEGER NOT NULL UNIQUE,
    tmdb_id INTEGER,
    tvdb_id INTEGER,
    imdb_id TEXT,
    overview TEXT,
    poster_path TEXT,
    backdrop_path TEXT,
    release_date TEXT,
    runtime INTEGER,
    vote_average REAL,
    vote_count INTEGER,
    genres TEXT, -- JSON array
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (media_item_id) REFERENCES media_items(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_video_metadata_media_item ON video_metadata(media_item_id);
CREATE INDEX IF NOT EXISTS idx_video_metadata_tmdb ON video_metadata(tmdb_id);
