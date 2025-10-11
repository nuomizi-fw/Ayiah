import { createAlova } from "alova";
import SolidHook from "alova/solid";
import fetchAdapter from "alova/fetch";

// Create Alova instance
export const alovaInstance = createAlova({
	baseURL: "/api",
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
