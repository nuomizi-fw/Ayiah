import {
	createRootRouteWithContext,
	Link,
	Outlet,
} from "@tanstack/solid-router";
import { QueryClient } from "@tanstack/solid-query";
import { TanStackRouterDevtools } from "@tanstack/solid-router-devtools";
import { SolidQueryDevtools } from "@tanstack/solid-query-devtools";
import { Suspense } from "solid-js";

export const Route = createRootRouteWithContext<{
	queryClient: QueryClient;
}>()({
	component: RootComponent,
});

function RootComponent() {
	return (
		<div class="min-h-screen bg-neutral-950 text-white">
			<nav class="border-b border-neutral-800 bg-neutral-900/50 backdrop-blur-sm sticky top-0 z-50">
				<div class="container mx-auto px-4 py-4">
					<div class="flex items-center justify-between">
						<Link
							to="/"
							class="text-2xl font-bold text-blue-500 hover:text-blue-400 transition-colors"
						>
							Ayiah
						</Link>
						<div class="flex gap-6">
							<Link
								to="/movie"
								class="hover:text-blue-400 transition-colors"
								activeProps={{ class: "text-blue-500" }}
							>
								Movies
							</Link>
							<Link
								to="/tv"
								class="hover:text-blue-400 transition-colors"
								activeProps={{ class: "text-blue-500" }}
							>
								TV Shows
							</Link>
							<Link
								to="/health"
								class="hover:text-blue-400 transition-colors"
								activeProps={{ class: "text-blue-500" }}
							>
								Health
							</Link>
						</div>
					</div>
				</div>
			</nav>
			<main class="container mx-auto px-4 py-8">
				<Suspense>
					<Outlet />
				</Suspense>
			</main>
			<SolidQueryDevtools buttonPosition="top-right" />
			<TanStackRouterDevtools position="bottom-right" />
		</div>
	);
}
