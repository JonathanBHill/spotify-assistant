# Installation

This guide will help you install and set up Spotify Assistant on your system.

## Prerequisites

- Rust and Cargo (latest stable version)
- Spotify account
- Spotify Developer account (for API access)

## Installation Steps

### 1. Clone the Repository

```bash
git clone https://github.com/JonathanBHill/spotify-assistant.git
cd spotify-assistant
```

### 2. Set Up Environment Variables

Create a `.env` file in the project root with the following variables:

```
SPOTIFY_CLIENT_ID=your_client_id
SPOTIFY_CLIENT_SECRET=your_client_secret
SPOTIFY_REDIRECT_URI=http://localhost:8888/callback
```

Replace `your_client_id` and `your_client_secret` with your Spotify Developer credentials.

### 3. Build the Project

```bash
cargo build --release
```

### 4. Run the Application

```bash
cargo run --release
```

## Troubleshooting

If you encounter any issues during installation, please check the following:

- Ensure your Rust toolchain is up to date
- Verify your Spotify Developer credentials
- Check that all required dependencies are installed
