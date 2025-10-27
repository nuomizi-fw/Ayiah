import { createFileRoute } from "@tanstack/solid-router";
import { Activity, Database, CheckCircle, XCircle } from "lucide-solid";
import { Show } from "solid-js";
import { getHealth } from "../api/health";

export const Route = createFileRoute("/health")({
	component: RouteComponent,
	loader: async () => {
		const response = await getHealth();
		return response;
	},
});

function RouteComponent() {
	const response = Route.useLoaderData();
	const healthData = () => response().data;

	const isHealthy = () => healthData()?.status === "healthy";
	const isDatabaseConnected = () => healthData()?.database === "connected";

	return (
		<div class="max-w-4xl mx-auto space-y-6">
			<div class="flex items-center gap-3">
				<Activity class="w-8 h-8 text-blue-500" />
				<h1 class="text-3xl font-bold">System Health</h1>
			</div>

			<div class="grid gap-4 md:grid-cols-2">
				{/* Overall Status */}
				<div class="bg-neutral-900 border border-neutral-800 rounded-lg p-6">
					<div class="flex items-center justify-between mb-4">
						<h2 class="text-xl font-semibold flex items-center gap-2">
							<Activity class="w-5 h-5" />
							Service Status
						</h2>
						<Show
							when={isHealthy()}
							fallback={
								<XCircle class="w-6 h-6 text-red-500" />
							}
						>
							<CheckCircle class="w-6 h-6 text-green-500" />
						</Show>
					</div>
					<div class="space-y-2">
						<div class="flex justify-between items-center">
							<span class="text-neutral-400">Status:</span>
							<span
								class={`font-semibold ${isHealthy() ? "text-green-500" : "text-red-500"
									}`}
							>
								{healthData()?.status || "unknown"}
							</span>
						</div>
					</div>
				</div>

				{/* Database Status */}
				<div class="bg-neutral-900 border border-neutral-800 rounded-lg p-6">
					<div class="flex items-center justify-between mb-4">
						<h2 class="text-xl font-semibold flex items-center gap-2">
							<Database class="w-5 h-5" />
							Database
						</h2>
						<Show
							when={isDatabaseConnected()}
							fallback={
								<XCircle class="w-6 h-6 text-red-500" />
							}
						>
							<CheckCircle class="w-6 h-6 text-green-500" />
						</Show>
					</div>
					<div class="space-y-2">
						<div class="flex justify-between items-center">
							<span class="text-neutral-400">Connection:</span>
							<span
								class={`font-semibold ${isDatabaseConnected() ? "text-green-500" : "text-red-500"
									}`}
							>
								{healthData()?.database || "unknown"}
							</span>
						</div>
					</div>
				</div>
			</div>

			{/* Additional Info */}
			<div class="bg-neutral-900 border border-neutral-800 rounded-lg p-6">
				<h2 class="text-xl font-semibold mb-4">System Information</h2>
				<div class="space-y-3 text-sm">
					<div class="flex justify-between items-center py-2 border-b border-neutral-800">
						<span class="text-neutral-400">API Endpoint:</span>
						<span class="font-mono text-neutral-300">/api/health</span>
					</div>
					<div class="flex justify-between items-center py-2 border-b border-neutral-800">
						<span class="text-neutral-400">Response Code:</span>
						<span class="font-mono text-neutral-300">{response().code}</span>
					</div>
					<div class="flex justify-between items-center py-2">
						<span class="text-neutral-400">Message:</span>
						<span class="font-mono text-neutral-300">{response().message}</span>
					</div>
				</div>
			</div>

			{/* Status Legend */}
			<div class="bg-neutral-900/50 border border-neutral-800 rounded-lg p-4">
				<h3 class="text-sm font-semibold text-neutral-400 mb-3">Status Legend</h3>
				<div class="grid gap-2 text-sm">
					<div class="flex items-center gap-2">
						<CheckCircle class="w-4 h-4 text-green-500" />
						<span class="text-neutral-300">Healthy / Connected</span>
					</div>
					<div class="flex items-center gap-2">
						<XCircle class="w-4 h-4 text-red-500" />
						<span class="text-neutral-300">Unhealthy / Disconnected</span>
					</div>
				</div>
			</div>
		</div>
	);
}
