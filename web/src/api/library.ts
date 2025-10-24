import type { ApiResponse } from "../types/api";
import type { LibraryResponse, MediaItemWithMetadata } from "../types/media";
import { alovaInstance } from "./client";

export const getMovies = () => {
	return alovaInstance.Get<ApiResponse<LibraryResponse>>("/library/movies");
};

export const getTvShows = () => {
	return alovaInstance.Get<ApiResponse<LibraryResponse>>("/library/tv");
};

export const getMediaItem = (id: number) => {
	return alovaInstance.Get<ApiResponse<MediaItemWithMetadata>>(
		`/library/items/${id}`,
	);
};

export const refreshMetadata = (id: number) => {
	return alovaInstance.Get<ApiResponse<string>>(`/library/items/${id}/refresh`);
};
