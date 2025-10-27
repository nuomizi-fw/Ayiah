import { createAlova } from "alova";
import fetchAdapter from "alova/fetch";
import SolidHook from "alova/solid";

// Create Alova instance
export const alovaInstance = createAlova({
	baseURL: import.meta.env.VITE_API_BASE_URL + "/api" || "http://127.0.0.1:7590" + "/api",
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
