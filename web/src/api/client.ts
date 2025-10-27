import { createAlova } from "alova";
import fetchAdapter from "alova/fetch";
import SolidHook from "alova/solid";

const API_BASE_URL =
	import.meta.env.VITE_API_BASE_URL || "http://127.0.0.1:7590";

// Create Alova instance
export const alovaInstance = createAlova({
	baseURL: `${API_BASE_URL}/api`,
	statesHook: SolidHook,
	requestAdapter: fetchAdapter(),
	responded: {
		onSuccess: async (response) => {
			const json = await response.json();
			if (json.code !== 200) {
				throw new Error(json.message || "Request failed");
			}
			return json;
		},
		onError: (error) => {
			console.error("API Error:", error);
			throw error;
		},
	},
});
