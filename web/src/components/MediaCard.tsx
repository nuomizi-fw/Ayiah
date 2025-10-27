import { RatingGroup } from "@ark-ui/solid/rating-group";
import { Calendar, Clock, Film, Star } from "lucide-solid";
import type { Component } from "solid-js";
import { Index, Show } from "solid-js";
import type { MediaItemWithMetadata } from "../types/media";

interface MediaCardProps {
	item: MediaItemWithMetadata;
	onClick?: () => void;
}

const MediaCard: Component<MediaCardProps> = (props: MediaCardProps) => {
	const posterUrl = () => {
		if (props.item.metadata?.poster_path) {
			return `https://image.tmdb.org/t/p/w500${props.item.metadata.poster_path}`;
		}
		return null;
	};

	const genres = () => {
		if (props.item.metadata?.genres) {
			try {
				return JSON.parse(props.item.metadata.genres) as string[];
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

	// Convert vote_average (0-10) to 5-star rating
	const starRating = () => {
		if (!props.item.metadata?.vote_average) return 0;
		return (props.item.metadata.vote_average / 10) * 5;
	};

	return (
		<div
			class="group cursor-pointer rounded-lg overflow-hidden bg-neutral-900 hover:bg-neutral-800 transition-all duration-300 hover:scale-[1.03] hover:shadow-2xl hover:shadow-blue-500/20 hover:ring-2 hover:ring-blue-500/30"
			onClick={() => props.onClick?.()}
		>
			{/* Poster Image Section */}
			<div class="relative aspect-[2/3] bg-neutral-800">
				<Show
					when={posterUrl()}
					fallback={
						<div class="w-full h-full flex items-center justify-center text-neutral-600 bg-gradient-to-br from-neutral-800 via-neutral-850 to-neutral-900">
							<div class="text-center p-6">
								<Film class="w-20 h-20 mx-auto mb-4 text-neutral-600 group-hover:text-neutral-500 transition-colors" />
								<div class="text-sm font-medium text-neutral-500 group-hover:text-neutral-400 transition-colors">
									No Poster Available
								</div>
							</div>
						</div>
					}
				>
					<img
						src={posterUrl()!}
						alt={props.item.title}
						class="w-full h-full object-cover"
						loading="lazy"
					/>
				</Show>

				{/* Rating Badge Overlay */}
				<Show when={props.item.metadata?.vote_average}>
					<div class="absolute top-2 right-2 bg-black/90 backdrop-blur-sm px-2.5 py-1.5 rounded-lg shadow-lg group-hover:bg-black/95 transition-colors">
						<RatingGroup.Root count={5} value={starRating()} readOnly>
							<RatingGroup.Control class="flex gap-0.5">
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
																	<Star class="w-3.5 h-3.5 text-neutral-600" />
																}
															>
																<Star class="w-3.5 h-3.5 text-yellow-400 fill-yellow-400 drop-shadow-sm" />
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
					</div>
				</Show>
			</div>

			{/* Content Section */}
			<div class="p-4">
				<h3 class="font-semibold text-lg mb-2 line-clamp-2 group-hover:text-blue-400 transition-colors">
					{props.item.title}
				</h3>

				<div class="space-y-2 text-sm text-neutral-400">
					{/* Release Year */}
					<Show when={props.item.metadata?.release_date}>
						<div class="flex items-center gap-2">
							<Calendar class="w-4 h-4" />
							<span>
								{new Date(props.item.metadata?.release_date!).getFullYear()}
							</span>
						</div>
					</Show>

					{/* Runtime */}
					<Show when={props.item.metadata?.runtime}>
						<div class="flex items-center gap-2">
							<Clock class="w-4 h-4" />
							<span>{formatRuntime(props.item.metadata?.runtime!)}</span>
						</div>
					</Show>

					{/* Genre Tags (Limited to 3) */}
					<Show when={genres().length > 0}>
						<div class="flex flex-wrap gap-1.5 mt-2">
							<Index each={genres().slice(0, 3)}>
								{(genre) => (
									<span class="px-2.5 py-1 bg-neutral-800 rounded-full text-xs font-medium text-neutral-300 group-hover:bg-neutral-700 group-hover:text-neutral-200 transition-all duration-200">
										{genre()}
									</span>
								)}
							</Index>
						</div>
					</Show>
				</div>
			</div>
		</div>
	);
};

export default MediaCard;
