// On importe les modules nécessaires pour la connexion réseau
use std::net::TcpStream;
use std::time::Duration;

// Fonction qui teste si un port est ouvert sur une adresse IP
// Retourne true si le port est ouvert, false sinon
fn est_port_ouvert(ip: &str, port: u16) -> bool {
    // On crée l'adresse complète (ex: "192.168.1.1:80")
    let adresse = format!("{}:{}", ip, port);
    
    // On essaie de se connecter au port avec un timeout de 500ms
    // (on réduit le timeout pour aller plus vite)
    match TcpStream::connect_timeout(&adresse.parse().unwrap(), Duration::from_millis(500)) {
        Ok(_) => {
            // Si la connexion réussit, le port est ouvert
            println!("✓ Port {} est OUVERT", port);
            true
        }
        Err(_) => {
            // Si la connexion échoue, le port est fermé (on n'affiche plus rien pour ne pas polluer)
            false
        }
    }
}

fn main() {
    println!("=== Port Scanner v0.2 ===");
    println!();
    
    // L'adresse IP à scanner (localhost = notre machine)
    let ip = "127.0.0.1";
    
    // On définit la plage de ports à scanner (de 1 à 1000)
    let port_debut = 1;
    let port_fin = 1000;
    
    println!("Scan de {} - Ports {} à {}", ip, port_debut, port_fin);
    println!();
    
    // Compteur pour savoir combien de ports sont ouverts
    let mut ports_ouverts = 0;
    
    // On boucle sur tous les ports de la plage
    for port in port_debut..=port_fin {
        // On teste chaque port
        if est_port_ouvert(ip, port) {
            ports_ouverts += 1;
        }
    }
    
    // On affiche le résumé à la fin
    println!();
    println!("=== Résumé ===");
    println!("Ports scannés : {}", port_fin - port_debut + 1);
    println!("Ports ouverts : {}", ports_ouverts);
}
