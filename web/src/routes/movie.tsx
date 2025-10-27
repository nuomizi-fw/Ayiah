import { createFileRoute, useNavigate } from "@tanstack/solid-router";
import { getMovies } from "../api/library";
import LibraryView from "../components/LibraryView";
import type { MediaItemWithMetadata } from "../types/media";

export const Route = createFileRoute("/movie")({
	component: RouteComponent,
	loader: async () => {
		const data = await getMovies();
		return data;
	},
});

function RouteComponent() {
	const navigate = useNavigate();
	const data = Route.useLoaderData();

	const handleItemClick = (item: MediaItemWithMetadata) => {
		navigate({ to: "/detail/$id", params: { id: String(item.id) } });
	};

	return (
		<div class="space-y-6">
			<h1 class="text-3xl font-bold">Movies</h1>
			<LibraryView
				mediaType="movie"
				onItemClick={handleItemClick}
				data={data}
			/>
		</div>
	);
}
