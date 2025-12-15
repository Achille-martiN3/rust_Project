// On importe les modules nécessaires pour la connexion réseau
use std::net::TcpStream;
use std::time::Duration;

// Fonction qui teste si un port est ouvert sur une adresse IP
// Retourne true si le port est ouvert, false sinon
fn est_port_ouvert(ip: &str, port: u16) -> bool {
    // On crée l'adresse complète (ex: "192.168.1.1:80")
    let adresse = format!("{}:{}", ip, port);
    
    // On essaie de se connecter au port avec le timeout donné
    match TcpStream::connect_timeout(&adresse.parse().unwrap(), Duration::from_secs(1)) {
        Ok(_) => {
            // Si la connexion réussit, le port est ouvert
            println!("✓ Port {} est OUVERT", port);
            true
        }
        Err(_) => {
            // Si la connexion échoue, le port est fermé
            println!("✗ Port {} est fermé", port);
            false
        }
    }
}

fn main() {
    println!("=== Port Scanner v0.1 ===");
    println!();
    
    // Pour l'instant, on teste juste un port sur localhost (notre propre machine)
    let ip = "127.0.0.1";
    let port = 80;
    
    println!("Scan de {}:{}", ip, port);
    println!();
    
    // On teste si le port est ouvert
    est_port_ouvert(ip, port);
}
