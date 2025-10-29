import { LoaderCircle } from "lucide-solid";
import type { Component } from "solid-js";

interface LoadingSpinnerProps {
	message?: string;
}

const LoadingSpinner: Component<LoadingSpinnerProps> = (
	props: LoadingSpinnerProps,
) => {
	return (
		<div class="flex flex-col items-center justify-center py-20">
			<LoaderCircle class="w-12 h-12 animate-spin text-blue-500 mb-4" />
			<p class="text-neutral-400">{props.message || "Loading..."}</p>
		</div>
	);
};

export default LoadingSpinner;
