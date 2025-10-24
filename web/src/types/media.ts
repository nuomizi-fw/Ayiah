export type MediaType = "movie" | "tv" | "comic" | "book";

export interface MediaItem {
	id: number;
	library_folder_id: number;
	media_type: MediaType;
	title: string;
	file_path: string;
	file_size: number;
	added_at: string;
	updated_at: string;
}

export interface VideoMetadata {
	id: number;
	media_item_id: number;
	tmdb_id: number | null;
	tvdb_id: number | null;
	imdb_id: string | null;
	overview: string | null;
	poster_path: string | null;
	backdrop_path: string | null;
	release_date: string | null;
	runtime: number | null;
	vote_average: number | null;
	vote_count: number | null;
	genres: string | null;
	created_at: string;
	updated_at: string;
}

export interface MediaItemWithMetadata {
	id: number;
	library_folder_id: number;
	media_type: MediaType;
	title: string;
	file_path: string;
	file_size: number;
	added_at: string;
	updated_at: string;
	metadata: VideoMetadata | null;
}

export interface LibraryResponse {
	items: MediaItemWithMetadata[];
	total: number;
}
