import type { Component } from "solid-js";
import { createSignal, onMount } from "solid-js";
import { getHealth, type HealthResponse } from "./api/health";

const App: Component = () => {
	const [health, setHealth] = createSignal<HealthResponse | null>(null);
	const [loading, setLoading] = createSignal(true);
	const [error, setError] = createSignal<string | null>(null);

	onMount(async () => {
		try {
			const response = await getHealth().send();
			setHealth(response.data ?? null);
		} catch (err) {
			setError(
				err instanceof Error ? err.message : "Failed to fetch health status",
			);
		} finally {
			setLoading(false);
		}
	});

	return (
		<div class="p-8">
			<h1 class="text-3xl font-bold mb-4">Ayiah Media Server</h1>

			<div class="mb-4">
				<h2 class="text-xl font-semibold mb-2">API Health Check</h2>
				{loading() && <p>Loading...</p>}
				{error() && <p class="text-red-500">Error: {error()}</p>}
				{health() && (
					<div class="bg-green-100 p-4 rounded">
						<p>Status: {health()!.status}</p>
						<p>Database: {health()!.database}</p>
					</div>
				)}
			</div>
		</div>
	);
};

export default App;
