import { Router, Route, A } from "@solidjs/router";
import { Film, House, Tv } from "lucide-solid";
import type { Component } from "solid-js";
import HomePage from "./pages/HomePage";
import MoviesPage from "./pages/MoviesPage";
import TvPage from "./pages/TvPage";
import DetailPage from "./pages/DetailPage";

const App: Component = () => {
	return (
		<Router>
			<div class="min-h-screen bg-neutral-950 text-white">
				<nav class="bg-neutral-900 border-b border-neutral-800 sticky top-0 z-50">
					<div class="max-w-7xl mx-auto px-6 py-4">
						<div class="flex items-center justify-between">
							<div class="flex items-center gap-8">
								<A
									href="/"
									class="text-2xl font-bold text-blue-500 hover:text-blue-400 transition-colors"
								>
									Ayiah
								</A>
								<div class="flex gap-4">
									<A
										href="/"
										class="flex items-center gap-2 px-4 py-2 rounded-lg transition-colors"
										activeClass="bg-blue-600 text-white"
										inactiveClass="text-neutral-400 hover:text-white hover:bg-neutral-800"
										end
									>
										<House class="w-5 h-5" />
										Home
									</A>
									<A
										href="/movies"
										class="flex items-center gap-2 px-4 py-2 rounded-lg transition-colors"
										activeClass="bg-blue-600 text-white"
										inactiveClass="text-neutral-400 hover:text-white hover:bg-neutral-800"
									>
										<Film class="w-5 h-5" />
										Movies
									</A>
									<A
										href="/tv"
										class="flex items-center gap-2 px-4 py-2 rounded-lg transition-colors"
										activeClass="bg-blue-600 text-white"
										inactiveClass="text-neutral-400 hover:text-white hover:bg-neutral-800"
									>
										<Tv class="w-5 h-5" />
										TV Shows
									</A>
								</div>
							</div>
						</div>
					</div>
				</nav>

				<main class="max-w-7xl mx-auto px-6 py-8">
					<Route path="/" component={HomePage} />
					<Route path="/movies" component={MoviesPage} />
					<Route path="/tv" component={TvPage} />
					<Route path="/detail/:id" component={DetailPage} />
				</main>
			</div>
		</Router>
	);
};

export default App;
