import { Dialog } from "@ark-ui/solid/dialog";
import { createListCollection, Select } from "@ark-ui/solid/select";
import { ChevronDown, FolderPlus, Loader2, X } from "lucide-solid";
import { createSignal, Index, Show } from "solid-js";
import { Portal } from "solid-js/web";
import { useMutation } from "@tanstack/solid-query";
import { createLibraryFolder } from "../api/library-folders";
import { useRouteQueryClient } from "../hooks/useRouteQueryClient";
import type { CreateLibraryFolderRequest } from "../types/library-folder";
import type { MediaType } from "../types/media";

export default function AddLibraryDialog() {
	const [open, setOpen] = createSignal(false);
	const [name, setName] = createSignal("");
	const [path, setPath] = createSignal("");
	const [mediaType, setMediaType] = createSignal<string[]>(["movie"]);

	// Get queryClient from router context
	const queryClient = useRouteQueryClient();

	const mediaTypeOptions = createListCollection({
		items: [
			{ label: "Movie", value: "movie" },
			{ label: "TV Show", value: "tv" },
			{ label: "Comic", value: "comic" },
			{ label: "Book", value: "book" },
		],
	});

	// Mutation for creating library folder
	const createMutation = useMutation(() => ({
		mutationFn: async (data: CreateLibraryFolderRequest) => {
			const result = await createLibraryFolder(data);
			return result;
		},
		onSuccess: () => {
			// Invalidate and refetch folders
			queryClient.invalidateQueries({ queryKey: ["library-folders"] });
			// Reset form
			setName("");
			setPath("");
			setMediaType(["movie"]);
			// Close dialog
			setOpen(false);
		},
	}));

	const handleSubmit = (e: Event) => {
		e.preventDefault();
		if (!name() || !path() || !mediaType()[0]) return;

		createMutation.mutate({
			name: name(),
			path: path(),
			media_type: mediaType()[0] as MediaType,
		});
	};

	return (
		<Dialog.Root open={open()} onOpenChange={(e) => setOpen(e.open)}>
			<Dialog.Trigger class="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors">
				<FolderPlus class="w-5 h-5" />
				Add Library Folder
			</Dialog.Trigger>

			<Portal>
				<Dialog.Backdrop class="fixed inset-0 bg-black/50 backdrop-blur-sm z-40" />
				<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
					<Dialog.Content class="bg-neutral-900 rounded-lg shadow-2xl max-w-md w-full border border-neutral-800">
						<div class="p-6">
							<div class="flex items-center justify-between mb-6">
								<Dialog.Title class="text-2xl font-bold">
									Add Library Folder
								</Dialog.Title>
								<Dialog.CloseTrigger class="p-2 hover:bg-neutral-800 rounded-lg transition-colors">
									<X class="w-5 h-5" />
								</Dialog.CloseTrigger>
							</div>

							<Dialog.Description class="text-neutral-400 mb-6">
								Add a new folder to scan for media files. The folder will be
								monitored for new content.
							</Dialog.Description>

							<form onSubmit={handleSubmit} class="space-y-4">
								{/* Name Input */}
								<div>
									<label
										for="folder-name"
										class="block text-sm font-medium mb-2"
									>
										Name
									</label>
									<input
										id="folder-name"
										type="text"
										value={name()}
										onInput={(e) => setName(e.currentTarget.value)}
										placeholder="My Movies"
										required
										class="w-full px-4 py-2 bg-neutral-800 border border-neutral-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
									/>
								</div>

								{/* Path Input */}
								<div>
									<label
										for="folder-path"
										class="block text-sm font-medium mb-2"
									>
										Path
									</label>
									<input
										id="folder-path"
										type="text"
										value={path()}
										onInput={(e) => setPath(e.currentTarget.value)}
										placeholder="/path/to/media/movies"
										required
										class="w-full px-4 py-2 bg-neutral-800 border border-neutral-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
									/>
									<p class="mt-1 text-xs text-neutral-500">
										The folder path on the server filesystem
									</p>
								</div>

								{/* Media Type Select */}
								<div>
									<label class="block text-sm font-medium mb-2">
										Media Type
									</label>
									<Select.Root
										collection={mediaTypeOptions}
										value={mediaType()}
										onValueChange={(details) => setMediaType(details.value)}
										positioning={{ sameWidth: true }}
									>
										<Select.Control>
											<Select.Trigger class="w-full flex items-center justify-between gap-2 px-4 py-2 bg-neutral-800 border border-neutral-700 rounded-lg hover:bg-neutral-750 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500">
												<Select.ValueText placeholder="Select media type" />
												<Select.Indicator>
													<ChevronDown class="w-4 h-4" />
												</Select.Indicator>
											</Select.Trigger>
										</Select.Control>
										<Portal>
											<Select.Positioner>
												<Select.Content class="bg-neutral-800 border border-neutral-700 rounded-lg shadow-xl overflow-hidden z-50 min-w-[200px]">
													<Select.ItemGroup>
														<Index each={mediaTypeOptions.items}>
															{(item) => (
																<Select.Item
																	item={item()}
																	class="flex items-center justify-between px-4 py-2 hover:bg-neutral-700 cursor-pointer transition-colors"
																>
																	<Select.ItemText>
																		{item().label}
																	</Select.ItemText>
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
								</div>

								{/* Error Message */}
								<Show when={createMutation.isError}>
									<div class="p-3 bg-red-500/10 border border-red-500/20 rounded-lg">
										<p class="text-sm text-red-400">
											{createMutation.error instanceof Error
												? createMutation.error.message
												: "Failed to create library folder"}
										</p>
									</div>
								</Show>

								{/* Action Buttons */}
								<div class="flex gap-3 pt-4">
									<button
										type="submit"
										disabled={
											createMutation.isPending ||
											!name() ||
											!path() ||
											!mediaType()[0]
										}
										class="flex-1 flex items-center justify-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-neutral-700 disabled:text-neutral-500 disabled:cursor-not-allowed rounded-lg font-semibold transition-colors"
									>
										<Show
											when={createMutation.isPending}
											fallback={
												<>
													<FolderPlus class="w-5 h-5" />
													Add Folder
												</>
											}
										>
											<Loader2 class="w-5 h-5 animate-spin" />
											Adding...
										</Show>
									</button>
									<Dialog.CloseTrigger
										type="button"
										class="px-4 py-2 bg-neutral-800 hover:bg-neutral-700 rounded-lg transition-colors"
									>
										Cancel
									</Dialog.CloseTrigger>
								</div>
							</form>
						</div>
					</Dialog.Content>
				</Dialog.Positioner>
			</Portal>
		</Dialog.Root>
	);
}
