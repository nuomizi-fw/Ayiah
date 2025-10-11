# Ayiah

Celestial (Ayiah), We're astral (Ayiou)

<html>
    <body>
        <img src="assets/logo.svg" alt="Ayiah Logo" width="1280" height="384">
    </body>
</html>

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/nuomizi-fw/Ayiah)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Project Status: WIP](https://img.shields.io/badge/status-work--in--progress-orange)](https://github.com/nuomizi-fw/Ayiah)

## Core Features

- **Unified Library:** A single, beautiful interface for all your media. No more switching between apps for movies, TV shows, comics, and books.
- **Full Automation:** Subscribe to the media you want, and Ayiah will take care of the rest. It automatically searches for new releases, sends them to your download client, and seamlessly imports them into your library.
- **High-Performance Backend:** Built with Rust and the Axum framework for a lightweight, fast, and secure foundation that can run efficiently even on low-powered hardware.
- **Modern, Snappy Frontend:** A beautiful and responsive user interface built with SolidStart, designed for a best-in-class user experience.
- **Built-in Readers:** Watch your videos, read your comics, and enjoy your ebooks directly in the browser with integrated, purpose-built readers.
- **Containerized & Easy to Deploy:** The entire application is orchestrated with Docker, allowing for a simple, one-command setup.

## Tech Stack

- **Backend:** [Rust](https://www.rust-lang.org/) with [Axum](https://github.com/tokio-rs/axum) & [Tokio](https://tokio.rs/)
- **Frontend:** [SolidStart](https://start.solidjs.com/) (TypeScript)
- **Database:** [DuckDB](https://www.duckdb.org/)
- **Containerization:** [Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/)

## Project Philosophy

The goal of Ayiah is to create the ultimate, integrated home media server. We prioritize:

1. **Elegance & Simplicity:** A clean, intuitive interface that makes managing a large library a pleasure.
2. **Performance & Efficiency:** A low-resource backend that is fast, reliable, and can run 24/7 without worry.
3. **Integration:** A seamless, "all-in-one" experience that removes the need to configure and maintain a complex stack of separate applications.

## Getting Started

> **Note:** This project is currently in the early stages of development. These instructions are the target for a first release.

Ayiah is designed to be run with Docker. Once the initial version is released, you will be able to get it running with these simple steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/nuomizi-fw/Ayiah.git
    cd Ayiah
    ```

2. Create a `docker-compose.yml` file (an example will be provided). You will need to configure your media library paths.
3. Start the application:

    ```bash
    docker-compose up -d
    ```

4. Open your web browser and navigate to `http://localhost:PORT` to access the Ayiah dashboard.

## Contributing

Contributions are welcome and encouraged! If you're interested in helping build the future of home media servers, please check out the `TODO.md` file for the project roadmap.

Feel free to open an issue to discuss a new feature or bug, or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
