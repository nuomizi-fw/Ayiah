import { createFileRoute, useNavigate } from "@tanstack/solid-router";
import { getMediaItem } from "../api/library";
import MediaDetail from "../components/MediaDetail";

export const Route = createFileRoute("/detail/$id")({
	component: RouteComponent,
	loader: async ({ params }) => {
		const data = await getMediaItem(Number(params.id));
		return data;
	},
});

function RouteComponent() {
	const navigate = useNavigate();
	const data = Route.useLoaderData();

	const handleBack = () => {
		navigate({ to: "/" });
	};

	return <MediaDetail data={data} onBack={handleBack} />;
}
