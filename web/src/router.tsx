import { createRouter as createTanstackSolidRouter } from "@tanstack/solid-router";
import { queryClient } from "./app";
import { routeTree } from "./routeTree.gen";

export function createRouter() {
	const router = createTanstackSolidRouter({
		routeTree,
		defaultPreload: "intent",
		defaultStaleTime: 5000,
		scrollRestoration: true,
		context: {
			queryClient,
		},
	});
	return router;
}

export const router = createRouter();

// Register things for typesafety
declare module "@tanstack/solid-router" {
	interface Register {
		router: ReturnType<typeof createRouter>;
	}
}
