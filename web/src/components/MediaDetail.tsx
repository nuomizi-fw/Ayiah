import { RatingGroup } from "@ark-ui/solid/rating-group";
import {
	ArrowLeft,
	Calendar,
	Clock,
	Info,
	Play,
	RefreshCw,
	Star,
} from "lucide-solid";
import type { Accessor, Component } from "solid-js";
import { createSignal, For, Index, Show } from "solid-js";
import { refreshMetadata } from "../api/library";
import type { ApiResponse } from "../types/api";
import type { MediaItemWithMetadata } from "../types/media";

interface MediaDetailProps {
	data: Accessor<ApiResponse<MediaItemWithMetadata>>;
	onBack: () => void;
}

const MediaDetail: Component<MediaDetailProps> = (props: MediaDetailProps) => {
	const [refreshing, setRefreshing] = createSignal(false);

	const item = () => props.data()?.data;

	const posterUrl = () => {
		if (item()?.metadata?.poster_path) {
			return `https://image.tmdb.org/t/p/w500${item()?.metadata?.poster_path}`;
		}
		return null;
	};

	const backdropUrl = () => {
		if (item()?.metadata?.backdrop_path) {
			return `https://image.tmdb.org/t/p/original${item()?.metadata?.backdrop_path}`;
		}
		return null;
	};

	const genres = () => {
		const genresString = item()?.metadata?.genres;
		if (genresString) {
			try {
				return JSON.parse(genresString) as string[];
			} catch {
				return [];
			}
		}
		return [];
	};

	const formatRuntime = (minutes: number | null) => {
		if (!minutes) return null;
		const hours = Math.floor(minutes / 60);
		const mins = minutes % 60;
		return hours > 0 ? `${hours}h ${mins}m` : `${mins}m`;
	};

	const formatFileSize = (bytes: number) => {
		const gb = bytes / (1024 * 1024 * 1024);
		return `${gb.toFixed(2)} GB`;
	};

	const handleRefresh = async () => {
		setRefreshing(true);
		try {
			const itemId = item()?.id;
			if (itemId) {
				await refreshMetadata(itemId);
				window.location.reload();
			}
		} catch (err) {
			console.error("Failed to refresh metadata:", err);
		} finally {
			setRefreshing(false);
		}
	};

	return (
		<div class="min-h-screen">
			<Show when={item()}>
				<div class="relative">
					<Show when={backdropUrl()}>
						<div class="absolute inset-0 h-[60vh]">
							<img
								src={backdropUrl()!}
								alt=""
								class="w-full h-full object-cover"
							/>
							<div class="absolute inset-0 bg-gradient-to-b from-transparent via-neutral-950/50 to-neutral-950" />
						</div>
					</Show>

					<div class="relative max-w-7xl mx-auto px-6 py-8">
						<button
							type="button"
							onClick={props.onBack}
							class="flex items-center gap-2 text-neutral-400 hover:text-white mb-8 transition-colors"
						>
							<ArrowLeft class="w-5 h-5" />
							Back to Library
						</button>

						<div class="flex flex-col md:flex-row gap-8 mb-8">
							<div class="flex-shrink-0">
								<div class="w-64 aspect-[2/3] rounded-lg overflow-hidden bg-neutral-900 shadow-2xl">
									<Show
										when={posterUrl()}
										fallback={
											<div class="w-full h-full flex items-center justify-center text-neutral-600">
												<div class="text-center">
													<div class="text-6xl mb-2">🎬</div>
													<div class="text-sm">No Poster</div>
												</div>
											</div>
										}
									>
										<img
											src={posterUrl()!}
											alt={item()?.title}
											class="w-full h-full object-cover"
										/>
									</Show>
								</div>
							</div>

							<div class="flex-1 space-y-6">
								<div>
									<h1 class="text-4xl font-bold mb-4">{item()?.title}</h1>

									<div class="flex flex-wrap gap-4 text-neutral-300 mb-4">
										<Show when={item()?.metadata?.release_date}>
											<div class="flex items-center gap-2">
												<Calendar class="w-5 h-5" />
												<span>
													{new Date(
														item()?.metadata?.release_date!,
													).getFullYear()}
												</span>
											</div>
										</Show>
										<Show when={item()?.metadata?.runtime}>
											<div class="flex items-center gap-2">
												<Clock class="w-5 h-5" />
												<span>{formatRuntime(item()?.metadata?.runtime!)}</span>
											</div>
										</Show>
										<Show when={item()?.metadata?.vote_average}>
											<div class="flex items-center gap-3">
												<RatingGroup.Root
													count={5}
													value={(item()?.metadata?.vote_average! / 10) * 5}
													readOnly
												>
													<RatingGroup.Control class="flex gap-1">
														<RatingGroup.Context>
															{(context) => (
																<Index each={context().items}>
																	{(index) => (
																		<RatingGroup.Item index={index()}>
																			<RatingGroup.ItemContext>
																				{(itemContext) => (
																					<Show
																						when={itemContext().highlighted}
																						fallback={
																							<Star class="w-5 h-5 text-yellow-400" />
																						}
																					>
																						<Star class="w-5 h-5 text-yellow-400 fill-yellow-400" />
																					</Show>
																				)}
																			</RatingGroup.ItemContext>
																		</RatingGroup.Item>
																	)}
																</Index>
															)}
														</RatingGroup.Context>
													</RatingGroup.Control>
												</RatingGroup.Root>
												<span class="font-semibold">
													{item()?.metadata?.vote_average?.toFixed(1)}
												</span>
												<span class="text-neutral-500">
													({item()?.metadata?.vote_count} votes)
												</span>
											</div>
										</Show>
									</div>

									<Show when={genres().length > 0}>
										<div class="flex flex-wrap gap-2 mb-6">
											<For each={genres()}>
												{(genre) => (
													<span class="px-3 py-1 bg-neutral-800 rounded-full text-sm">
														{genre}
													</span>
												)}
											</For>
										</div>
									</Show>
								</div>

								<div class="flex gap-3">
									<button
										type="button"
										class="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors"
									>
										<Play class="w-5 h-5" />
										Play
									</button>
									<button
										type="button"
										onClick={handleRefresh}
										disabled={refreshing()}
										class="flex items-center gap-2 px-4 py-3 bg-neutral-800 hover:bg-neutral-700 rounded-lg transition-colors disabled:opacity-50"
									>
										<RefreshCw
											class={`w-5 h-5 ${refreshing() ? "animate-spin" : ""}`}
										/>
										Refresh
									</button>
								</div>

								<Show when={item()?.metadata?.overview}>
									<div>
										<h2 class="text-xl font-semibold mb-2 flex items-center gap-2">
											<Info class="w-5 h-5" />
											Overview
										</h2>
										<p class="text-neutral-300 leading-relaxed">
											{item()?.metadata?.overview}
										</p>
									</div>
								</Show>
							</div>
						</div>

						<div class="bg-neutral-900/80 backdrop-blur-sm rounded-lg p-6 space-y-4">
							<h2 class="text-xl font-semibold mb-4">Media Information</h2>
							<div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
								<div>
									<span class="text-neutral-400">Type:</span>
									<span class="ml-2 capitalize">{item()?.media_type}</span>
								</div>
								<div>
									<span class="text-neutral-400">File Size:</span>
									<span class="ml-2">{formatFileSize(item()!.file_size)}</span>
								</div>
								<Show when={item()?.added_at}>
									<div>
										<span class="text-neutral-400">Added:</span>
										<span class="ml-2">
											{new Date(item()!.added_at).toLocaleDateString()}
										</span>
									</div>
								</Show>
								<Show when={item()?.metadata?.tmdb_id}>
									<div>
										<span class="text-neutral-400">TMDB ID:</span>
										<span class="ml-2">{item()?.metadata?.tmdb_id}</span>
									</div>
								</Show>
								<Show when={item()?.metadata?.imdb_id}>
									<div>
										<span class="text-neutral-400">IMDB ID:</span>
										<span class="ml-2">{item()?.metadata?.imdb_id}</span>
									</div>
								</Show>
								<div class="md:col-span-2">
									<span class="text-neutral-400">File Path:</span>
									<span class="ml-2 text-xs text-neutral-500 break-all">
										{item()?.file_path}
									</span>
								</div>
							</div>
						</div>
					</div>
				</div>
			</Show>
		</div>
	);
};

export default MediaDetail;
