import type { QueryClient } from "@tanstack/solid-query";
import { useRouteContext } from "@tanstack/solid-router";

/**
 * Hook to access the QueryClient from the router context.
 * This provides a type-safe way to access the queryClient that's
 * provided in the router configuration.
 *
 * @returns QueryClient instance from router context
 *
 * @example
 * ```tsx
 * function MyComponent() {
 *   const queryClient = useRouteQueryClient();
 *
 *   const mutation = createMutation(() => ({
 *     mutationFn: async (data) => await createItem(data),
 *     onSuccess: () => {
 *       queryClient.invalidateQueries({ queryKey: ['items'] });
 *     },
 *   }));
 * }
 * ```
 */
export function useRouteQueryClient(): QueryClient {
	const context = useRouteContext({
		from: "__root__",
	});

	// useRouteContext returns an Accessor, so we need to call it
	return context().queryClient;
}
