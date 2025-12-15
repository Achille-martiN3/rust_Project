# Port Scanner 🔍

Un scanner de ports réseau rapide et multi-thread développé en Rust.

## 📋 Description

Ce port scanner permet de détecter rapidement les ports ouverts sur une machine cible. Il utilise le multi-threading pour scanner plusieurs ports simultanément, ce qui accélère considérablement le processus. Le programme identifie également les services courants associés à chaque port ouvert (HTTP, SSH, FTP, etc.).

## ⚙️ Fonctionnalités

- ✅ Scan rapide multi-thread
- ✅ Détection des services courants (HTTP, SSH, FTP, RDP, MySQL, etc.)
- ✅ Interface en ligne de commande complète avec options personnalisables
- ✅ Timeout configurable pour chaque connexion
- ✅ Mode verbose pour afficher tous les ports testés
- ✅ Plage de ports personnalisable

## 🛠️ Compilation

Assurez-vous d'avoir [Rust](https://www.rust-lang.org/tools/install) installé sur votre système.

```bash
# Cloner le projet
git clone <votre-repo>
cd rust_Project

# Compiler le projet
cargo build --release

# L'exécutable sera disponible dans target/release/
```

## 🚀 Utilisation

### Exemples d'utilisation

```bash
# Scan par défaut (localhost, ports 1-1000)
cargo run

# Afficher l'aide
cargo run -- --help

# Scanner une IP spécifique
cargo run -- --ip 192.168.1.1

# Scanner une plage de ports spécifique
cargo run -- --port-debut 1 --port-fin 100

# Scanner avec un timeout personnalisé (en ms)
cargo run -- --timeout 500

# Mode verbose (affiche tous les ports testés)
cargo run -- --verbose

# Exemple complet
cargo run -- --ip 192.168.1.1 --port-debut 20 --port-fin 3389 --timeout 300
```

### Options disponibles

| Option | Short | Description | Défaut |
|--------|-------|-------------|--------|
| `--ip` | `-i` | Adresse IP à scanner | 127.0.0.1 |
| `--port-debut` | `-s` | Port de début de la plage | 1 |
| `--port-fin` | `-e` | Port de fin de la plage | 1000 |
| `--timeout` | `-t` | Timeout en millisecondes | 200 |
| `--verbose` | `-v` | Afficher tous les ports testés | false |
| `--help` | `-h` | Afficher l'aide | - |

## 📊 Services détectés

Le scanner identifie automatiquement les services suivants :

- **FTP** (20, 21) - Transfert de fichiers
- **SSH** (22) - Connexion sécurisée
- **Telnet** (23) - Connexion non sécurisée
- **SMTP** (25) - Envoi d'emails
- **DNS** (53) - Résolution de noms
- **HTTP** (80) - Web
- **POP3** (110) - Réception d'emails
- **NetBIOS** (139) - Partage Windows
- **IMAP** (143) - Réception d'emails
- **HTTPS** (443) - Web sécurisé
- **SMB** (445) - Partage de fichiers Windows
- **RPC** (135) - Appels de procédures Windows
- **MySQL** (3306) - Base de données
- **PostgreSQL** (5432) - Base de données
- **RDP** (3389) - Bureau à distance
- **MS SQL** (1433) - Base de données Microsoft
- **VNC** (5900) - Contrôle à distance
- Et bien d'autres...

## ⚠️ Avertissement

**IMPORTANT :** Cet outil est développé à des fins éducatives et de test de sécurité uniquement.

- ❌ **NE PAS** utiliser sur des systèmes sans autorisation explicite
- ❌ **NE PAS** utiliser pour des activités malveillantes
- ✅ Utilisez uniquement sur vos propres machines ou avec permission écrite
- ✅ Respectez les lois en vigueur dans votre pays

L'utilisation non autorisée de scanners de ports peut être illégale et entraîner des poursuites judiciaires. L'auteur n'est pas responsable de l'utilisation abusive de cet outil.

## 🧠 Architecture technique

Le projet utilise :
- **Rust** pour la performance et la sécurité mémoire
- **std::thread** pour le multi-threading
- **Arc et Mutex** pour le partage de données entre threads
- **TcpStream** pour les connexions TCP
- **clap** pour l'interface en ligne de commande

## 📝 Développement par étapes

Le projet a été développé en 5 parties :
1. ✅ Fonction de base pour scanner un port
2. ✅ Scan de plage de ports (1-1000)
3. ✅ Multi-threading pour améliorer les performances
4. ✅ Détection des services courants
5. ✅ Interface CLI complète avec options

## 📄 Licence

Projet éducatif - À utiliser de manière responsable uniquement.

## 👨‍💻 Auteur

Développé dans le cadre d'un projet académique en cybersécurité.