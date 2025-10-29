import type { ApiResponse } from "../types/api";
import type {
	CreateLibraryFolderRequest,
	LibraryFolder,
	ScanResponse,
} from "../types/library-folder";
import { alovaInstance } from "./client";

export const getLibraryFolders = () => {
	return alovaInstance.Get<ApiResponse<LibraryFolder[]>>("/library-folders");
};

export const getLibraryFolder = (id: number) => {
	return alovaInstance.Get<ApiResponse<LibraryFolder>>(
		`/library-folders/${id}`,
	);
};

export const createLibraryFolder = (data: CreateLibraryFolderRequest) => {
	return alovaInstance.Post<ApiResponse<LibraryFolder>>(
		"/library-folders",
		data,
	);
};

export const deleteLibraryFolder = (id: number) => {
	return alovaInstance.Delete<ApiResponse<string>>(`/library-folders/${id}`);
};

export const scanLibraryFolder = (id: number) => {
	return alovaInstance.Post<ApiResponse<ScanResponse>>(
		`/library-folders/${id}/scan`,
	);
};

export const scanAllLibraryFolders = () => {
	return alovaInstance.Post<ApiResponse<ScanResponse[]>>(
		"/library-folders/scan-all",
	);
};
