import { createAlova } from "alova";
import fetchAdapter from "alova/fetch";
import SolidHook from "alova/solid";

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
