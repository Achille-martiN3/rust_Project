// On importe les modules nécessaires pour la connexion réseau
use std::net::TcpStream;
use std::time::Duration;
// On importe les modules pour le multi-threading
use std::sync::{Arc, Mutex};
use std::thread;

// Fonction qui teste si un port est ouvert sur une adresse IP
// Retourne true si le port est ouvert, false sinon
fn est_port_ouvert(ip: &str, port: u16) -> bool {
    // On crée l'adresse complète (ex: "192.168.1.1:80")
    let adresse = format!("{}:{}", ip, port);
    
    // On essaie de se connecter au port avec un timeout de 200ms
    // (encore plus court pour profiter du multi-threading)
    match TcpStream::connect_timeout(&adresse.parse().unwrap(), Duration::from_millis(200)) {
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
    println!("=== Port Scanner v0.3 (Multi-thread) ===");
    println!();
    
    // L'adresse IP à scanner (localhost = notre machine)
    let ip = "127.0.0.1";
    
    // On définit la plage de ports à scanner
    let port_debut = 1;
    let port_fin = 1000;
    
    println!("Scan de {} - Ports {} à {}", ip, port_debut, port_fin);
    println!("Utilisation de threads pour accélérer le scan...");
    println!();
    
    // On crée une liste partagée entre tous les threads pour stocker les ports ouverts
    // Arc = Atomic Reference Counted (permet de partager entre threads)
    // Mutex = verrou pour éviter que 2 threads modifient la liste en même temps
    let ports_ouverts = Arc::new(Mutex::new(Vec::new()));
    
    // Liste pour stocker nos threads
    let mut handles = vec![];
    
    // On crée un thread pour chaque port
    for port in port_debut..=port_fin {
        // On clone les variables pour que chaque thread ait sa copie
        let ip_clone = ip.to_string();
        let ports_clone = Arc::clone(&ports_ouverts);
        
        // On crée un nouveau thread
        let handle = thread::spawn(move || {
            // Chaque thread teste un port
            if est_port_ouvert(&ip_clone, port) {
                // Si le port est ouvert, on l'ajoute à la liste
                // lock() permet de verrouiller la liste le temps d'ajouter le port
                ports_clone.lock().unwrap().push(port);
            }
        });
        
        // On garde une référence au thread
        handles.push(handle);
    }
    
    // On attend que tous les threads aient terminé
    for handle in handles {
        handle.join().unwrap();
    }
    
    // On récupère la liste finale des ports ouverts
    let resultat = ports_ouverts.lock().unwrap();
    
    // On affiche le résumé à la fin
    println!();
    println!("=== Résumé ===");
    println!("Ports scannés : {}", port_fin - port_debut + 1);
    println!("Ports ouverts : {}", resultat.len());
}
