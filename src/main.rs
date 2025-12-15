// On importe les modules nécessaires pour la connexion réseau
use std::net::TcpStream;
use std::time::Duration;
// On importe les modules pour le multi-threading
use std::sync::{Arc, Mutex};
use std::thread;
// On importe clap pour gérer les arguments de ligne de commande
use clap::Parser;

// Structure qui définit les arguments que l'utilisateur peut passer en ligne de commande
// Parser va automatiquement créer l'aide et parser les arguments
#[derive(Parser, Debug)]
#[command(name = "Port Scanner")]
#[command(about = "Un scanner de ports rapide et multi-thread", long_about = None)]
struct Args {
    /// Adresse IP à scanner (ex: 192.168.1.1)
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,

    /// Port de début de la plage
    #[arg(short = 's', long, default_value = "1")]
    port_debut: u16,

    /// Port de fin de la plage
    #[arg(short = 'e', long, default_value = "1000")]
    port_fin: u16,

    /// Timeout en millisecondes pour chaque connexion
    #[arg(short, long, default_value = "200")]
    timeout: u64,

    /// Afficher tous les ports testés (même fermés)
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

// Fonction qui retourne le nom du service associé à un port
// Par exemple : port 80 = HTTP, port 22 = SSH, etc.
fn obtenir_nom_service(port: u16) -> &'static str {
    match port {
        20 => "FTP (données)",
        21 => "FTP (contrôle)",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP (email)",
        53 => "DNS",
        80 => "HTTP (web)",
        110 => "POP3 (email)",
        135 => "RPC Windows",
        139 => "NetBIOS",
        143 => "IMAP (email)",
        443 => "HTTPS (web sécurisé)",
        445 => "SMB (partage fichiers)",
        1433 => "MS SQL Server",
        3306 => "MySQL",
        3389 => "RDP (Bureau à distance)",
        5432 => "PostgreSQL",
        5900 => "VNC",
        8080 => "HTTP alternatif",
        8443 => "HTTPS alternatif",
        _ => "Service inconnu",
    }
}

// Fonction qui teste si un port est ouvert sur une adresse IP
// Retourne true si le port est ouvert, false sinon
fn est_port_ouvert(ip: &str, port: u16, timeout: u64, verbose: bool) -> bool {
    // On crée l'adresse complète (ex: "192.168.1.1:80")
    let adresse = format!("{}:{}", ip, port);
    
    // On essaie de se connecter au port avec le timeout donné par l'utilisateur
    match TcpStream::connect_timeout(&adresse.parse().unwrap(), Duration::from_millis(timeout)) {
        Ok(_) => {
            // Si la connexion réussit, le port est ouvert
            // On affiche aussi le service associé
            println!(" Port {} est OUVERT - {}", port, obtenir_nom_service(port));
            true
        }
        Err(_) => {
            // Si la connexion échoue, le port est fermé
            // On n'affiche que si le mode verbose est activé
            if verbose {
                println!(" Port {} est fermé", port);
            }
            false
        }
    }
}

fn main() {
    // On parse les arguments de ligne de commande
    let args = Args::parse();
    
    println!("=== Port Scanner v1.0 (Multi-thread + Services + CLI) ===");
    println!();
    
    // On utilise les arguments fournis par l'utilisateur
    let ip = &args.ip;
    let port_debut = args.port_debut;
    let port_fin = args.port_fin;
    let timeout = args.timeout;
    let verbose = args.verbose;
    
    // Vérification que la plage de ports est valide
    if port_debut > port_fin {
        eprintln!("Erreur : le port de début doit être inférieur au port de fin !");
        return;
    }
    
    println!("Scan de {} - Ports {} à {}", ip, port_debut, port_fin);
    println!("Timeout : {}ms | Mode verbose : {}", timeout, if verbose { "OUI" } else { "NON" });
    println!("Utilisation de threads pour accélérer le scan...");
    println!();
    
    // On crée une liste partagée entre tous les threads pour stocker les ports ouverts
    // Arc = Atomic Reference Counted (permet de partager entre threads)
    // Mutex = verrou pour éviter que 2 threads modifient la liste en même temps
    let ports_ouverts = Arc::new(Mutex::new(Vec::<(u16, &str)>::new()));
    
    // Liste pour stocker nos threads
    let mut handles = vec![];
    
    // On crée un thread pour chaque port
    for port in port_debut..=port_fin {
        // On clone les variables pour que chaque thread ait sa copie
        let ip_clone = ip.clone();
        let ports_clone = Arc::clone(&ports_ouverts);
        
        // On crée un nouveau thread
        let handle = thread::spawn(move || {
            // Chaque thread teste un port
            if est_port_ouvert(&ip_clone, port, timeout, verbose) {
                // Si le port est ouvert, on l'ajoute à la liste avec son service
                // lock() permet de verrouiller la liste le temps d'ajouter le port
                let service = obtenir_nom_service(port);
                ports_clone.lock().unwrap().push((port, service));
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
    let mut resultat = ports_ouverts.lock().unwrap();
    // On trie les ports par ordre croissant pour un affichage propre
    resultat.sort_by_key(|k| k.0);
    
    // On affiche le résumé à la fin
    println!();
    println!("=== Résumé ===");
    println!("Ports scannés : {}", port_fin - port_debut + 1);
    println!("Ports ouverts : {}", resultat.len());
    println!();
    
    // On affiche la liste des ports ouverts avec leurs services
    if !resultat.is_empty() {
        println!("Liste des ports ouverts :");
        for (port, service) in resultat.iter() {
            println!("   Port {} - {}", port, service);
        }
    } else {
        println!("Aucun port ouvert trouvé.");
    }
}
