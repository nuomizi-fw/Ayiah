import { createListCollection, Select } from "@ark-ui/solid/select";
import { useRequest } from "alova/client";
import { ChevronDown, Grid, List, Search } from "lucide-solid";
import type { Component } from "solid-js";
import { createMemo, createSignal, For, Index, Show } from "solid-js";
import { Portal } from "solid-js/web";
import { getMovies, getTvShows } from "../api/library";
import type { MediaItemWithMetadata, MediaType } from "../types/media";
import MediaCard from "./MediaCard";

interface LibraryViewProps {
	mediaType: MediaType;
	onItemClick: (item: MediaItemWithMetadata) => void;
}

const LibraryView: Component<LibraryViewProps> = (props) => {
	const [searchQuery, setSearchQuery] = createSignal("");
	const [viewMode, setViewMode] = createSignal<"grid" | "list">("grid");
	const [sortBy, setSortBy] = createSignal<string[]>(["date"]);

	const sortOptions = createListCollection({
		items: [
			{ label: "Recently Added", value: "date" },
			{ label: "Title", value: "title" },
			{ label: "Rating", value: "rating" },
		],
	});

	const apiCall = () => {
		return props.mediaType === "movie" ? getMovies() : getTvShows();
	};

	const { data, loading, error } = useRequest(apiCall, {
		initialData: { data: { items: [], total: 0 } },
	});

	const filteredItems = createMemo(() => {
		let items = data()?.data?.items || [];

		if (searchQuery()) {
			const query = searchQuery().toLowerCase();
			items = items.filter((item: MediaItemWithMetadata) =>
				item.title.toLowerCase().includes(query),
			);
		}

		items = [...items].sort(
			(a: MediaItemWithMetadata, b: MediaItemWithMetadata) => {
				const sortValue = sortBy()[0] || "date";
				switch (sortValue) {
					case "title":
						return a.title.localeCompare(b.title);
					case "rating":
						return (
							(b.metadata?.vote_average || 0) - (a.metadata?.vote_average || 0)
						);
					default:
						return (
							new Date(b.added_at).getTime() - new Date(a.added_at).getTime()
						);
				}
			},
		);

		return items;
	});

	return (
		<div class="space-y-6">
			<div class="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between">
				<div class="flex-1 max-w-md relative">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-neutral-400" />
					<input
						type="text"
						placeholder="Search library..."
						value={searchQuery()}
						onInput={(e) => setSearchQuery(e.currentTarget.value)}
						class="w-full pl-10 pr-4 py-2 bg-neutral-900 border border-neutral-800 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
					/>
				</div>

				<div class="flex gap-2">
					<Select.Root
						collection={sortOptions}
						value={sortBy()}
						onValueChange={(details) => setSortBy(details.value)}
						positioning={{ sameWidth: true }}
					>
						<Select.Control class="min-w-[180px]">
							<Select.Trigger class="flex items-center justify-between gap-2 px-4 py-2 bg-neutral-900 border border-neutral-800 rounded-lg hover:bg-neutral-800 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500">
								<Select.ValueText placeholder="Sort by" />
								<Select.Indicator>
									<ChevronDown class="w-4 h-4" />
								</Select.Indicator>
							</Select.Trigger>
						</Select.Control>
						<Portal>
							<Select.Positioner>
								<Select.Content class="bg-neutral-900 border border-neutral-800 rounded-lg shadow-xl overflow-hidden z-50">
									<Select.ItemGroup>
										<Index each={sortOptions.items}>
											{(item) => (
												<Select.Item
													item={item()}
													class="flex items-center justify-between px-4 py-2 hover:bg-neutral-800 cursor-pointer transition-colors"
												>
													<Select.ItemText>{item().label}</Select.ItemText>
													<Select.ItemIndicator class="text-blue-500">
														✓
													</Select.ItemIndicator>
												</Select.Item>
											)}
										</Index>
									</Select.ItemGroup>
								</Select.Content>
							</Select.Positioner>
						</Portal>
					</Select.Root>

					<div class="flex bg-neutral-900 border border-neutral-800 rounded-lg overflow-hidden">
						<button
							type="button"
							onClick={() => setViewMode("grid")}
							class={`px-3 py-2 transition-colors ${
								viewMode() === "grid"
									? "bg-blue-600 text-white"
									: "hover:bg-neutral-800"
							}`}
						>
							<Grid class="w-5 h-5" />
						</button>
						<button
							type="button"
							onClick={() => setViewMode("list")}
							class={`px-3 py-2 transition-colors ${
								viewMode() === "list"
									? "bg-blue-600 text-white"
									: "hover:bg-neutral-800"
							}`}
						>
							<List class="w-5 h-5" />
						</button>
					</div>
				</div>
			</div>

			<Show when={loading()}>
				<div class="flex items-center justify-center py-20">
					<div class="text-center">
						<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4" />
						<p class="text-neutral-400">Loading library...</p>
					</div>
				</div>
			</Show>

			<Show when={error()}>
				<div class="bg-red-900/20 border border-red-900 rounded-lg p-4">
					<p class="text-red-400">Failed to load library: {error()?.message}</p>
				</div>
			</Show>

			<Show when={!loading() && !error()}>
				<div class="space-y-4">
					<div class="flex items-center justify-between">
						<p class="text-neutral-400">
							{filteredItems().length}{" "}
							{filteredItems().length === 1 ? "item" : "items"}
						</p>
					</div>

					<Show
						when={filteredItems().length > 0}
						fallback={
							<div class="text-center py-20">
								<div class="text-6xl mb-4">🎬</div>
								<p class="text-xl text-neutral-400 mb-2">No items found</p>
								<p class="text-sm text-neutral-500">
									{searchQuery()
										? "Try adjusting your search"
										: "Your library is empty"}
								</p>
							</div>
						}
					>
						<div
							class={
								viewMode() === "grid"
									? "grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-6"
									: "space-y-4"
							}
						>
							<For each={filteredItems()}>
								{(item) => (
									<MediaCard
										item={item}
										onClick={() => props.onItemClick(item)}
									/>
								)}
							</For>
						</div>
					</Show>
				</div>
			</Show>
		</div>
	);
};

export default LibraryView;
