# ConnectionInternat

Ce programme permet de se connecter au réseau interne de l'internat du Lycée du Parc. Le [contrôleur du réseau](https://controller.access.network) étant ancien, son chiffrement obsolète et son certificat invalide, il est incompatible avec les navigateurs les plus récents.

La démarche proposée par le lycée est de télécharger la version antérieure d'un navigateur (ils proposent le navigateur Firefox version 130.0), et de positionner la variable `security.tls.version.min` à `1` dans la configuration du navigateur, avant de se rendre sur le [site](https://controller.access.network) pour se connecter.

Cette manipulation a pour but de rendre l'utilisation du protocole TLSv1.0 par le navigateur possible, alors qu'elle a été volontairement été retiré pour des raisons de sécurité entre autres (voir [ici](https://blog.mozilla.org/security/2018/10/15/removing-old-versions-of-tls/) et [là](https://hacks.mozilla.org/2020/02/its-the-boot-for-tls-1-0-and-tls-1-1/)). Par ailleurs, le mécanisme de connexion nécessite d'avoir cette page ouverte pendant toute la durée de sa session de connexion, ce qui s'avère être contraignant puisque que les navigateurs modernes ferment les onglets inutilisés.

Pour ces raison, il est difficile de trouver un moyen simple et efficace pour se connecter réseau interne. Or l'ancienneté du certificat et de la méthode de cryptage du contrôleur nous met dans l'impossibilité de faire un programme simple pour s'y connecter.

Ainsi, nous avons créé ce logiciel, qui possède une interface graphique, pour se connecter au réseau interne de l'internat. Il suffira de le lancer et de le garder ouvert, et il se connectera automatiquement au réseau, gardant la connection active.

> **Notes**
>
> 1. Si vous venez d'une autre école, ou chose similaire, et que vous voulez qu'on travaille pour l'adapter à votre réseau, n'hésitez pas à nous contacter, à travers une issue sur ce repo, ou un mail à l'adresse dans la description de [@itsvyle](https://github.com/itsvyle).
> 2. L'ancienne version du script, plus simple et réalisée spécifiquement pour Linux, est toujours disponible dans le répertoir [legacy](/legacy/README.md).

## Installation

Le logiciel est pour l'instant compatible avec Linux et Windows. Le support mac est possible et pourra être implémenté, cependant nous n'avons pas de mac pour le tester et ainsi n'avons pas pu le finir - si vous avez un mac et que vous voulez utiliser l'application, n'hésitez pas à nous contacter, par un mail à l'adresse dans la description de [@itsvyle](https://github.com/itsvyle).

### Windows

Requiert _a priori_ windows 10 ou plus récent; on ne peut pas garantir le fonctionnement sur des versions plus anciennes.

<details>
  <summary>Instructions pour télécharger sur Windows</summary>

## 1. Télécharger l'installeur

Cliquez ici pour télécharger l'installeur: [installer-ConnectionInternat-windows.exe](https://github.com/ntillier/ConnectionInternat/releases/latest/download/installer-ConnectionInternat-windows.exe)

## 2. Exécuter l'installeur

Ici, windows vous informera que le programme n'est pas reconnu - c'est normal, étant donné que nous ne sommes pas une entreprise reconnue par Microsoft. Cependant, vous pouvez cliquer sur "Plus d'informations" et "Exécuter quand même" (voir ci-dessous).

**Attention**: vous aurez besoin d'être connecté à internet pour que l'installeur puisse télécharger le logiciel.

### 1. Cliquez sur "Informations supplémentaires"

![Cliquer sur "Informations supplémentaires"](./.github/assets/windows-protect-step-1.png)

### 2. Cliquez sur "Exécuter quand même"

![Cliquez sur "Exécuter quand même"](./.github/assets/windows-protect-step-2.png)

Pour ce qui est de la sécurité de l'installeur et du programme, vous pouvez consulter le code source, qui est ouvert et disponible sur ce repo; les fichiers générés sont créés par github directement à partir du code source, et sont donc sûrs.

## 3. Attendre que l'installation se fasse

Un terminal de texte s'ouvrira, et installera le programme. Une fois que le programme est téléchargé, vous verrez un message de confirmation, et pourrez appuyer sur entrée pour quitter l'installeur.

</details>

---

### <ins>Utiliser le programme sur Windows</ins>

L'installeur créera des raccourcis "ConnectionInternat" pour le programme à deux endroits:

- Sur le bureau
- Dans le menu démarrer

Vous pouvez lancer le programme en cliquant sur un de ces raccourcis.

Une fenêtre s'ouvrira, dans un terminal; elle doit rester ouverte en permanence pour que le programme fonctionne. Vous pouvez la minimiser, mais ne la fermez pas.

---

### Linux

Fonctionne normalement sur n'importe quelle distribution Linux, ne requiert aucune dépendance spécifique.

<details>
  <summary>Instructions pour télécharger sur Linux</summary>

Sur linux, vous avez plus de choix: vous pouvez utiliser l'installeur, ou télécharger directement les fichiers et les exécuter, depuis la [page releases](https://github.com/ntillier/ConnectionInternat/releases/latest/). Si vous utilisez cette dernière méthode, faites attention à bien garder l'éxécutable backend (`ConnectionInternat-backend.exe`) dans le même dossier que l'exécutable frontend (`ConnectionInternat`).

Pour utiliser l'installeur (recommandé), suivez les instructions ci-dessous.

## 1. Télécharger l'installeur

Téléchargez l'installeur en cliquant sur le lien ci-dessous:

- Pour Linux 64 bits sur x86: [installer-ConnectionInternat-linux-amd64](https://github.com/ntillier/ConnectionInternat/releases/latest/download/installer-ConnectionInternat-linux-amd64)
- Pour Linux 64 bits sur arm: [installer-ConnectionInternat-linux-arm64](https://github.com/ntillier/ConnectionInternat/releases/latest/download/installer-ConnectionInternat-linux-arm64)

## 2. Exécuter l'installeur

Ouvrez un terminal, et naviguez jusqu'au dossier où vous avez téléchargé l'installeur. Vous pouvez ensuite exécuter l'installeur en tapant la commande suivante:

```bash
chmod +x installer-ConnectionInternat-linux-<votre architecture>
./installer-ConnectionInternat-linux-<votre architecture>
```

## 3. Attendre que l'installation se fasse

Un terminal de texte s'ouvrira, et installera le programme. Une fois que le programme est téléchargé, vous verrez un message de confirmation, et pourrez appuyer sur entrée pour quitter l'installeur.

</details>

---

### <ins>Utiliser le programme sur Linux</ins>

Si vous avez utilisé l'installeur:

- Le proramme sera installé dans le dossier `~/.local/ConnectionInternat`.
- Un racourci (symlink) sera créé dans `~/ConnectionInternat`, qui vous permettra de lancer le programme simplement en tapant `~/ConnectionInternat` dans un terminal.

Sinon, éxécutez le programme `ConnectionInternat` depuis le dossier où vous avez téléchargé les fichiers.

---

## Utilisation

Pour lancer le programme, voir la section "Utiliser le programme" pour votre système d'exploitation.

À tout moment, vous pouvez fermer le programme en faisant `Échap` ou `q` sur votre clavier.

Une fois le programme lancé, vous verrez une fenêtre qui ressemble à ceci:

![Initial vue](./.github/assets/usage-1.png)

Pour tous les menus, vous pouvez utiliser les flèches du clavier pour naviguer, et la touche `Entrée` pour valider votre choix.

---

### Se connecter

Pour se connecter la première fois, il suffira de choisir "Rentrer ses identifiants", puis de:

1. Rentrer son nom d'utilisateur
2. Appuyer sur `Entrée`
3. Rentrer son mot de passe
4. Appuyer sur `Entrée`

Les identifiants seront sauvegardés, et vous n'aurez plus à les rentrer à chaque fois (pour les nerds: ils sont sauvegardés dans `~/.internat-connection.txt`)

---

### Se déconnecter

Avant de vous déconnecter du réseau ethernet, il est recommandé de se déconnecter manuellement: cela est possible en cliquant sur le boutton du menu, ou en appuyant sur `Échap` ou `q` sur votre clavier.

---

### Se reconnecter

Si vous avez été déconnecté du réseau, vous pouvez vous reconnecter en cliquant sur le boutton du menu qui s'affichera.

## Design

- Mettre tlsv1.0
- mettre nom du cryptage, pour seo
