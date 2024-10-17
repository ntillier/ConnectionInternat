#!/bin/bash


# Variables globales
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
conf="$SCRIPT_DIR/openssl_config.cnf"
credentials="$SCRIPT_DIR/identifiants.txt"
pattern='/(?:.+)passwordDigest(?:.+?)value(?:.+?)"([^"]+)"(?:.+)/g'

# On récupère les identifiants
username="$(sed '1q;d' $credentials)"
password="$(sed '2q;d' $credentials)"

# Quelques fonctions
login() {
    echo "Connection avec le login $1"
    result="$(OPENSSL_CONF="$conf" curl -s -k --tlsv1 'https://controller.access.network/portal_api.php' -X POST -H 'Content-Type: application/x-www-form-urlencoded' --data-raw "action=authenticate&login=$1&password=$2&policy_accept=false")"

    digest=$(jq -r '.user.passwordDigest.value' <<< "$result")

    if [[ -z "$digest" ]];
    then
        echo "Identifiants invalides"
        exit 1
        fi

        echo "Connecté !"
}

logout() {
    result="$(OPENSSL_CONF="$conf" curl -s -k "https://controller.access.network/portal_api.php" -X POST -H "Content-Type: application/x-www-form-urlencoded" --data-raw "action=disconnect&login=$1&password_digest=$2")"

    code=$(jq -r '.info.code' <<< "$result")

    if [[ "$code" = "disconnect_success" ]];
    then
        echo "Déconnecté"
    else
        echo "Erreur lors de la déconnexion"
        exit 1
    fi
}

# Commandes
if [[ "$1" = "logout" ]];
then
    logout
    exit 1
fi

# On récupère le cookie de la session
# Il semble que ce ne soit pas nécessaire :)
#echo "Récupération des cookies..."
#curl -o /dev/null -s --cookie-jar $cookies  http://controller.access.network/101/portal

# On se connecte (en supposant que les identifiants sont corrects)
login "$username" "$password"

# Refresh
while sleep 50;
do
	epoch=$(date -d "Oct 21 1973" +%s)
	result=$(OPENSSL_CONF="$conf" curl -k -s "https://controller.access.network/portal_api.php" -X POST -H "Content-Type: application/x-www-form-urlencoded" --data-raw "action=refresh&login=$username&password_digest=$digest&time=$epoch")

	echo "Session mise à jour."
done

# Logout
logout "$username" "$digest"
