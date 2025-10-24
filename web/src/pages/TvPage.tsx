import { useNavigate } from "@solidjs/router";
import LibraryView from "../components/LibraryView";
import type { MediaItemWithMetadata } from "../types/media";

export default function TvPage() {
    const navigate = useNavigate();

    const handleItemClick = (item: MediaItemWithMetadata) => {
        navigate(`/detail/${item.id}`);
    };

    return (
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">TV Shows</h1>
            <LibraryView mediaType="tv" onItemClick={handleItemClick} />
        </div>
    );
}
