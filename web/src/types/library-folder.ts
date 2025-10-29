import type { MediaType } from "./media";

export interface LibraryFolder {
	id: number;
	name: string;
	path: string;
	media_type: MediaType;
	enabled: boolean;
	created_at: string;
	updated_at: string;
}

export interface CreateLibraryFolderRequest {
	name: string;
	path: string;
	media_type: MediaType;
}

export interface ScanResult {
	added: number;
	updated: number;
	skipped: number;
	errors: number;
}

export interface ScanResponse {
	folder: LibraryFolder;
	result: ScanResult;
}
