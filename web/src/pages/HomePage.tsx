import { A } from "@solidjs/router";
import { Film, Tv } from "lucide-solid";

export default function HomePage() {
    return (
        <div class="space-y-8">
            <div class="text-center py-12">
                <h1 class="text-5xl font-bold mb-4">Welcome to Ayiah</h1>
                <p class="text-xl text-neutral-400 mb-8">Your personal media server</p>
                <div class="flex gap-4 justify-center">
                    <A
                        href="/movies"
                        class="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors"
                    >
                        <Film class="w-5 h-5" />
                        Browse Movies
                    </A>
                    <A
                        href="/tv"
                        class="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors"
                    >
                        <Tv class="w-5 h-5" />
                        Browse TV Shows
                    </A>
                </div>
            </div>
        </div>
    );
}
