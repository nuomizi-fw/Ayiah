import { useMutation } from "@tanstack/solid-query";
import { createFileRoute } from "@tanstack/solid-router";
import {
	Book,
	FileImage,
	Film,
	Folder,
	Loader2,
	RefreshCw,
	Trash2,
	Tv,
} from "lucide-solid";
import { For, Show } from "solid-js";
import {
	deleteLibraryFolder,
	getLibraryFolders,
	scanLibraryFolder,
} from "../api/library-folders";
import AddLibraryDialog from "../components/AddLibraryDialog";
import { useRouteQueryClient } from "../hooks/useRouteQueryClient";
import type { LibraryFolder } from "../types/library-folder";

export const Route = createFileRoute("/libraries")({
	component: RouteComponent,
	loader: async () => {
		// Preload library folders data before rendering
		return await getLibraryFolders();
	},
});

function RouteComponent() {
	// Get preloaded data from loader
	const data = Route.useLoaderData();

	// Get queryClient from router context for cache invalidation
	const queryClient = useRouteQueryClient();

	// Mutation for deleting library folders
	const deleteMutation = useMutation(() => ({
		mutationFn: async (id: number) => {
			const result = await deleteLibraryFolder(id);
			return result;
		},
		onSuccess: () => {
			// Invalidate cache to trigger refetch
			queryClient.invalidateQueries({ queryKey: ["library-folders"] });
		},
	}));

	// Mutation for scanning library folders
	const scanMutation = useMutation(() => ({
		mutationFn: async (id: number) => {
			const result = await scanLibraryFolder(id);
			return result;
		},
		onSuccess: () => {
			// Invalidate cache to trigger refetch
			queryClient.invalidateQueries({ queryKey: ["library-folders"] });
		},
	}));

	const handleDelete = (id: number, name: string) => {
		if (
			confirm(
				`Are you sure you want to delete the library folder "${name}"? This will not delete the actual files.`,
			)
		) {
			deleteMutation.mutate(id);
		}
	};

	const handleScan = (id: number) => {
		scanMutation.mutate(id);
	};

	const getMediaTypeIcon = (mediaType: string) => {
		switch (mediaType) {
			case "movie":
				return <Film class="w-5 h-5" />;
			case "tv":
				return <Tv class="w-5 h-5" />;
			case "comic":
				return <FileImage class="w-5 h-5" />;
			case "book":
				return <Book class="w-5 h-5" />;
			default:
				return <Folder class="w-5 h-5" />;
		}
	};

	const getMediaTypeColor = (mediaType: string) => {
		switch (mediaType) {
			case "movie":
				return "bg-blue-500/10 text-blue-400 border-blue-500/20";
			case "tv":
				return "bg-purple-500/10 text-purple-400 border-purple-500/20";
			case "comic":
				return "bg-pink-500/10 text-pink-400 border-pink-500/20";
			case "book":
				return "bg-green-500/10 text-green-400 border-green-500/20";
			default:
				return "bg-neutral-500/10 text-neutral-400 border-neutral-500/20";
		}
	};

	return (
		<div class="space-y-6">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-3xl font-bold mb-2">Library Folders</h1>
					<p class="text-neutral-400">
						Manage your media library folders and scan for new content
					</p>
				</div>
				<AddLibraryDialog />
			</div>

			<div class="space-y-4">
				<Show
					when={data()?.data && data()?.data!.length > 0}
					fallback={
						<div class="text-center py-20">
							<div class="text-6xl mb-4">📁</div>
							<p class="text-xl text-neutral-400 mb-2">
								No library folders yet
							</p>
							<p class="text-sm text-neutral-500 mb-6">
								Add your first library folder to start building your media
								collection
							</p>
							<AddLibraryDialog />
						</div>
					}
				>
					<div class="grid gap-4">
						<For each={data()?.data}>
							{(folder: LibraryFolder) => (
								<div class="bg-neutral-900 border border-neutral-800 rounded-lg p-6 hover:border-neutral-700 transition-colors">
									<div class="flex items-start justify-between gap-4">
										<div class="flex-1 min-w-0">
											<div class="flex items-center gap-3 mb-2">
												<div
													class={`p-2 rounded-lg border ${getMediaTypeColor(folder.media_type)}`}
												>
													{getMediaTypeIcon(folder.media_type)}
												</div>
												<div class="flex-1 min-w-0">
													<h3 class="text-lg font-semibold truncate">
														{folder.name}
													</h3>
													<p class="text-sm text-neutral-400 truncate">
														{folder.path}
													</p>
												</div>
											</div>

											<div class="flex items-center gap-4 text-sm text-neutral-400 mt-3">
												<span class="capitalize">
													Type: {folder.media_type}
												</span>
												<span>•</span>
												<span>
													{folder.enabled ? (
														<span class="text-green-400">Enabled</span>
													) : (
														<span class="text-red-400">Disabled</span>
													)}
												</span>
												<span>•</span>
												<span>
													Added:{" "}
													{new Date(folder.created_at).toLocaleDateString()}
												</span>
											</div>
										</div>

										<div class="flex gap-2 shrink-0">
											<button
												type="button"
												onClick={() => handleScan(folder.id)}
												disabled={scanMutation.isPending}
												class="flex items-center gap-2 px-3 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-neutral-700 disabled:text-neutral-500 rounded-lg transition-colors text-sm"
												title="Scan for new media"
											>
												<Show
													when={scanMutation.isPending}
													fallback={<RefreshCw class="w-4 h-4" />}
												>
													<Loader2 class="w-4 h-4 animate-spin" />
												</Show>
												Scan
											</button>
											<button
												type="button"
												onClick={() => handleDelete(folder.id, folder.name)}
												disabled={deleteMutation.isPending}
												class="flex items-center gap-2 px-3 py-2 bg-red-600 hover:bg-red-700 disabled:bg-neutral-700 disabled:text-neutral-500 rounded-lg transition-colors text-sm"
												title="Delete library folder"
											>
												<Trash2 class="w-4 h-4" />
											</button>
										</div>
									</div>
								</div>
							)}
						</For>
					</div>
				</Show>
			</div>
		</div>
	);
}
