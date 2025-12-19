// On importe les modules nécessaires pour la connexion réseau
use std::fs::File;
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
// On importe les modules pour le multi-threading
use std::sync::{Arc, Mutex};
use std::thread;
// On importe clap pour gérer les arguments de ligne de commande
use clap::Parser;
// Pour la sérialisation JSON
use serde::Serialize;

// Arguments de ligne de commande
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

    /// Liste de ports personnalisée (ex: 22,80,443)
    #[arg(long)]
    ports: Option<String>,

    /// Ne pas afficher chaque port ouvert (seulement le résumé)
    #[arg(long, default_value = "false")]
    quiet: bool,

    /// Exporter les ports ouverts en JSON vers un fichier
    #[arg(long)]
    json: Option<String>,
}

// Structure pour exporter en JSON
#[derive(Serialize)]
struct ExportOpenPort {
    port: u16,
    service: String,
}

#[derive(Serialize)]
struct ExportResult {
    ip: String,
    ports_scanned: usize,
    open_ports: Vec<ExportOpenPort>,
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

// la je parse la liste de ports que l'utilisateur peut donner genre "22,80,443"
// je split par virgule et je garde que les trucs valides
fn parse_ports_list(ports: &str) -> Vec<u16> {
    ports
        .split(',')
        .filter_map(|p| p.trim().parse::<u16>().ok())
        .collect()
}

// bon la c'est la fonction principale pour tester un port
// en gros on essaye de se connecter et si ca marche le port est ouvert
// j'ai rajouté les parametres pour afficher ou pas selon les options
fn est_port_ouvert(ip: &str, port: u16, timeout: u64, show_open: bool, show_closed: bool) -> bool {
    // on construit l'adresse complete genre "192.168.1.1:80" ou "scanme.nmap.org:80"
    let adresse = format!("{}:{}", ip, port);
    // on resout le DNS si c'est un hostname (genre scanme.nmap.org)
    let socket_addr = match adresse.to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(addr) => addr,
            None => return false,
        },
        Err(_) => return false,
    };
    // on tente la connexion avec le timeout qu'on a choisi
    match TcpStream::connect_timeout(&socket_addr, Duration::from_millis(timeout)) {
        Ok(_) => {
            if show_open {
                println!(" Port {} est OUVERT - {}", port, obtenir_nom_service(port));
            }
            true
        }
        Err(_) => {
            if show_closed {
                println!(" Port {} est fermé", port);
            }
            false
        }
    }
}

// ici c'est pour exporter les resultats en json si l'user veut
// j'utilise serde pour serialiser la structure et j'ecris dans un fichier
fn exporter_json(path: &str, data: &ExportResult) {
    match serde_json::to_vec_pretty(data) {
        Ok(buf) => {
            if let Err(err) = File::create(path).and_then(|mut f| f.write_all(&buf)) {
                eprintln!("Erreur lors de l'écriture du JSON: {}", err);
            } else {
                println!("Résultats exportés dans {}", path);
            }
        }
        Err(err) => eprintln!("Erreur de sérialisation JSON: {}", err),
    }
}

fn main() {
    // on recupere les arguments que l'user a passé
    let args = Args::parse();

    println!("=== Port Scanner v1.1 (CLI étendue) ===");
    println!();

    // la on regarde si l'user a donné une liste custom de ports ou pas
    // si oui on utilise sa liste sinon on prend la plage debut-fin
    let mut ports: Vec<u16> = if let Some(liste) = &args.ports {
        let parsed = parse_ports_list(liste);
        if parsed.is_empty() {
            eprintln!("Aucun port valide dans --ports. Exemple: --ports 22,80,443");
            return;
        }
        parsed
    } else {
        if args.port_debut > args.port_fin {
            eprintln!("Erreur : le port de début doit être inférieur au port de fin !");
            return;
        }
        (args.port_debut..=args.port_fin).collect()
    };

    let ports_scanned = ports.len();
    println!("Scan de {} - {} ports à tester", args.ip, ports_scanned);
    println!("Timeout : {}ms | Verbose : {} | Quiet : {}", args.timeout, if args.verbose { "OUI" } else { "NON" }, if args.quiet { "OUI" } else { "NON" });
    println!("Utilisation de threads pour accélérer le scan...");
    println!();

    // la c'est la liste ou on stocke les ports ouverts qu'on trouve
    // le Arc et Mutex c'est pour que tous les threads puissent ecrire dedans sans probleme
    let ports_ouverts = Arc::new(Mutex::new(Vec::<(u16, String)>::new()));

    // on stocke les handles de threads pour attendre qu'ils finissent
    let mut handles = vec![];
    // ces variables c'est pour savoir si on affiche les ports ouverts/fermés
    let show_open = !args.quiet;
    let show_closed = args.verbose;

    // bon la on lance un thread par port, pour aller plus vite
    for port in ports.drain(..) {
        let ip_clone = args.ip.clone();
        let ports_clone = Arc::clone(&ports_ouverts);
        let timeout = args.timeout;
        let show_open = show_open;
        let show_closed = show_closed;

        let handle = thread::spawn(move || {
            if est_port_ouvert(&ip_clone, port, timeout, show_open, show_closed) {
                let service = obtenir_nom_service(port).to_string();
                ports_clone.lock().unwrap().push((port, service));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut resultat = ports_ouverts.lock().unwrap();
    resultat.sort_by_key(|k| k.0);

    println!();
    println!("=== Résumé ===");
    println!("Ports scannés : {}", ports_scanned);
    println!("Ports ouverts : {}", resultat.len());
    println!();

    if !resultat.is_empty() {
        println!("Liste des ports ouverts :");
        for (port, service) in resultat.iter() {
            println!("   Port {} - {}", port, service);
        }
    } else {
        println!("Aucun port ouvert trouvé.");
    }

    // si l'user a demandé un export json on le fait ici
    if let Some(path) = &args.json {
        let export = ExportResult {
            ip: args.ip.clone(),
            ports_scanned,
            open_ports: resultat
                .iter()
                .map(|(p, s)| ExportOpenPort {
                    port: *p,
                    service: s.clone(),
                })
                .collect(),
        };
        exporter_json(path, &export);
    }
}
