import { alovaInstance } from "./client";

export interface HealthResponse {
	status: string;
	database: string;
}

export interface ApiResponse<T> {
	code: number;
	message: string;
	data?: T;
}

export const getHealth = () => {
	return alovaInstance.Get<ApiResponse<HealthResponse>>("/health");
};
