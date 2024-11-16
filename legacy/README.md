> Ce script concerne la version originelle de la commande, écrite en bash.

Ce script a pour but d'automatiser la connection de l'ordinateur au réseau ethernet du lycée (notamment de l'internat).Étant donné que le routeur utilise le protocol TSL 1.0, qui n'est plus supporté par les dernières versions de navigateurs, ce script permet de connecter automatiquement au réseau sans baisser la version minimale de sécurité du navigateur.
Il est censé fonctionner sur toutes les distributions de linux.

# Prérequis

- Avoir cURL installé. Cela pourra être vérifié avec la commande `curl -V` - elle devrait indiquer que OpenSSL est en utilisation
- Avoir `jq` installé, pour décoder du json. La commande `jq -r '.t' <<< '{"t":1}'` est censé renvoyer `1`. ([Découvrir jq](https://jqlang.github.io/jq/))

# Installation

- Cloner ce dépôt de code, ou bien les fichiers `./script.sh` et `openssl_config.cnf`.
- Créer un fichier nommé `identifiants.txt`, avec l'identifiant fournit sur la première ligne et le mot de passe sur la deuxième ligne.
- Rendre `script.sh` exécutable, avec la commande `chmod -x script.sh`.
- L'exécuter, avec `./script.sh` par exemple.

# Conseil

Pour éviter de devoir se déplacer dans le répertoire du script, il est possible de créer un alias.

- Créez un fichier `.bash_aliases` ([plus d'informations](https://www.cyberciti.biz/faq/create-permanent-bash-alias-linux-unix/))
- Ajoutez la ligne `alias connect="~/chemin/acces/script.sh"`
- Éxecutez la commande `connect` dans un terminal pour vous connecter
