# Passwortgenerator

Ein einfacher Passwortgenerator, der zufällige Passwörter mit einer angegebenen Anzahl von Zeichen generiert. Sie können auch komplexe Passwörter mit Sonderzeichen erstellen und die generierten Passwörter in die Zwischenablage kopieren.

## Funktionen

- Generiert ein zufälliges Passwort mit einer angegebenen Anzahl von Zeichen.
- Optionale Komplexität durch Hinzufügen von Sonderzeichen.
- Möglichkeit, das Passwort in die Zwischenablage zu kopieren.

## Verwendung

### Installation

Stellen Sie sicher, dass Sie Rust und Cargo installiert haben. Klonen Sie das Repository und führen Sie die folgenden Befehle aus:

```bash
cargo build
```

### Ausführen

Um das Programm auszuführen, verwenden Sie den folgenden Befehl:

```bash
cargo run -- --help
```

### Befehlszeilenargumente

- `-n`, `--numbers`: Anzahl der Zeichen im Passwort (Standard: 12)
- `-x`, `--complex`: Fügen Sie Sonderzeichen hinzu (Standard: false)
- `-c`, `--copy`: Kopieren Sie das Passwort in die Zwischenablage (Standard: false)

### Beispiel

```bash
cargo run -- -n 16 -x -c
```

## Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert. Siehe die [LICENSE](LICENSE) Datei für Details.