# experiment

This repository is organized as a simple monorepo with:

- **frontend** – a Dart web application that presents a login page and, on successful authentication, displays a home page noting the project was created by AI.
- **backend** – a Rust service using SQLite for persistence. It exposes a `/login` endpoint used by the frontend.

## Running

### Backend
```bash
cd backend
cargo run
```

The server listens on `http://localhost:3000` and stores its data in `backend/data/app.db`.

### Frontend
The frontend requires the Dart SDK. After installing Dart, compile and serve:
```bash
cd frontend
# build main.dart to JavaScript
dart run build_runner build
# then serve index.html with any static file server
```

The login page will send credentials to the backend and navigate to the home page on success.
