import { RouterProvider } from "@tanstack/solid-router";
import { router } from "./router";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";

import "./app.css";

export const queryClient = new QueryClient();

export default function App() {
	return (
		<QueryClientProvider client={queryClient}>
			<RouterProvider router={router} />
		</QueryClientProvider>
	);
}
