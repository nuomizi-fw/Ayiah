import { useNavigate } from "@solidjs/router";
import LibraryView from "../components/LibraryView";
import type { MediaItemWithMetadata } from "../types/media";

export default function MoviesPage() {
    const navigate = useNavigate();

    const handleItemClick = (item: MediaItemWithMetadata) => {
        navigate(`/detail/${item.id}`);
    };

    return (
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">Movies</h1>
            <LibraryView mediaType="movie" onItemClick={handleItemClick} />
        </div>
    );
}
