import type { ApiResponse } from "../types/api";
import { alovaInstance } from "./client";

export interface HealthResponse {
	status: string;
	database: string;
}

export const getHealth = () => {
	return alovaInstance.Get<ApiResponse<HealthResponse>>("/health");
};
