import { CircleAlert } from "lucide-solid";
import type { Component } from "solid-js";

interface ErrorMessageProps {
	title?: string;
	message: string;
	onRetry?: () => void;
}

const ErrorMessage: Component<ErrorMessageProps> = (
	props: ErrorMessageProps,
) => {
	return (
		<div class="flex flex-col items-center justify-center py-20">
			<div class="bg-red-500/10 border border-red-500/20 rounded-lg p-8 max-w-md">
				<div class="flex items-center gap-3 mb-4">
					<CircleAlert class="w-8 h-8 text-red-500" />
					<h2 class="text-xl font-semibold text-red-400">
						{props.title || "Error Loading Data"}
					</h2>
				</div>
				<p class="text-neutral-300 mb-6">{props.message}</p>
				{props.onRetry && (
					<button
						type="button"
						onClick={props.onRetry}
						class="w-full px-4 py-2 bg-red-600 hover:bg-red-700 rounded-lg transition-colors"
					>
						Try Again
					</button>
				)}
			</div>
		</div>
	);
};

export default ErrorMessage;
