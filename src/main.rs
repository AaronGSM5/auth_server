use axum::{Router, routing::{get, post}};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use axum::extract::{State, Path};
use axum::Server;
use bcrypt::{hash, verify, BcryptError};
use dotenv::dotenv;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use uuid::Uuid;
use chrono::{Duration, Utc};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // AppState mit Mutex für den Benutzer-Hashmap
    let app_state = Arc::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    // Initialisieren des Routers
    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(app_state.clone());

    // Setze Adresse und Listener
    let addr = "127.0.0.1:3000".to_string();
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("Server läuft unter http://{}", addr);
    
    // Starte den Axum-Server
    axum::serve(listener, app).await.unwrap();
}

// AppState mit Mutex, um den Zugang zur Benutzer-Hashmap zu synchronisieren
struct AppState {
    users: Mutex<HashMap<String, User>>,
}

#[derive(Deserialize, Serialize, Clone)]
struct RegisterPayload {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct LoginPayload {
    username: String,
    password: String,
}

#[derive(Clone)]
struct User {
    username: String,
    password_hash: String,
}

// Registrierung der Benutzer
async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    let hashed_password = match hash_password(&payload.password).await {
        Ok(hash) => hash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Fehler beim Hashen der Passwortes").into_response(),
    };

    let user = User {
        username: payload.username,
        password_hash: hashed_password,
    };

    let mut users = state.users.lock().unwrap();
    users.insert(user.username.clone(), user);

    (StatusCode::CREATED, "Benutzer registriert").into_response()
}

// Login des Benutzers
async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let users = state.users.lock().unwrap();

    match users.get(&payload.username) {
        Some(user) => match verify_password(&payload.password, &user.password_hash).await {
            Ok(true) => {
                // Token erstellen und zurückgeben
                let token = create_jwt(&user).await;
                Json(token).into_response()
            }
            Ok(false) => (StatusCode::UNAUTHORIZED, "Falsches Passwort").into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Fehler bei der Passwortüberprüfung").into_response(),
        },
        None => (StatusCode::NOT_FOUND, "Benutzer nicht gefunden").into_response(),
    }
}

// Passwort mit bcrypt hashen
async fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, 12)
}

// Passwort überprüfen
async fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}

// JWT-Token erstellen
async fn create_jwt(user: &User) -> String {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY muss gesetzt sein");
    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        sub: user.username.clone(),
        exp: expiration.timestamp() as usize,
    };

    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &secret_key.as_bytes()).unwrap()
}

// Struktur für JWT-Claims
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
