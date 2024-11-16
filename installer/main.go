package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"runtime"
)

// const RepoPath = "ntillier/ConnectionInternat"
// const ProgramName = "ConnectionInternat"

const RepoPath = "sharkdp/bat"
const ProgramName = "bat"

func main() {
	runningOS := runtime.GOOS
	arch := runtime.GOARCH

	if arch == "arm64" {
		arch = "aarch64"
	} else if arch == "amd64" {
		arch = "x86_64"
	}

	if runningOS == "darwin" {
		panic("L'application n'est pas supportée sur mac pour l'instant, meme si elle devrait etre facile à implémenter - merci de contacter les créateurs")
	}
	latestVersion, err := getLatestVersion()

	if err != nil {
		fmt.Println(err)
		panic("Error getting latest version")
	}
	fmt.Printf("En train d'installer la version: %s", latestVersion)

	suffix := "unknown-linux-musl" // just install musl version to avoid hassle
	if runningOS == "darwin" {
		suffix = "apple-darwin"
	} else if runningOS == "windows" {
		suffix = "pc-windows-msvc"
	}

	f := fmt.Sprintf("%s-%s-%s-%s", ProgramName, latestVersion, arch, suffix)
	fmt.Println("En train d'installer le fichier: ", f)

	// This will be a directory called ConnectionInternat cause who cares about versionning...
	installLocation := ""
	if runningOS == "linux" {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			panic("Couldn't find user home directory!")
		}
		installLocation = filepath.Join(homeDir, ".local", ProgramName)
	} else if runningOS == "windows" {
		localAppData, ok := os.LookupEnv("LOCALAPPDATA")
		if !ok {
			fmt.Println("LOCALAPPDATA environment variable not found")
			panic("Couldn't find LOCALAPPDATA environment variable")
		}
		installLocation = filepath.Join(localAppData, ProgramName)
	} else {
		panic("unknown OS: " + runningOS)
	}

	installURL := fmt.Sprintf("https://github.com/%s/releases/download/%s/%s", RepoPath, latestVersion, f)
	if runningOS == "windows" {
		installURL += ".zip"
	} else {
		installURL += ".tar.gz"
	}
	// TRES IMPORTANT - NE RIEN SUPPRIMER JUSQU'à CE QU'ON AIT TÉLÉCHARGÉ
	fmt.Printf("En train de télécharger %s\n", installURL)

	tempDir, err := downloadArchive(installURL)
	if err != nil {
		fmt.Println("error downloading archive: ", err)
		panic("couldn't download archive")
	}

	fmt.Printf("Téléchargé et extrait dans %s\n", tempDir)

	fmt.Printf("En train d'enlever l'ancienne installation si elle existe, à %s\n", installLocation)
	err = os.RemoveAll(installLocation)
	if err != nil && !os.IsNotExist(err) {
		fmt.Println("Error cleaning directory:", err)
		return
	}

	tempDir = filepath.Join(tempDir, f)
	if _, err := os.Stat(tempDir); os.IsNotExist(err) {
		panic(fmt.Sprintf("coudn't find the correct directory in, it does not exist %s", tempDir))
	}

	err = MoveFolder(tempDir, installLocation, true)
	if err != nil {
		fmt.Println(err)
		panic(fmt.Sprintf("couldn't copy files from %s to %s", tempDir, installLocation))
	}

	fmt.Println("En train de supprimer le dossier temporaire")
	os.RemoveAll(tempDir)
}

type GitHubRelease struct {
	TagName string `json:"tag_name"`
}

func getLatestVersion() (string, error) {
	ctx := context.Background()
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, "https://api.github.com/repos/"+RepoPath+"/releases/latest", nil)
	if err != nil {
		fmt.Println("Error creating request:", err)
		return "", err
	}

	req.Header.Set("Accept", "application/vnd.github.v3+json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("Error fetching data:", err)
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		fmt.Println("Error fetching latest release, got status: ", resp.StatusCode)
		return "", errors.New("can't find latest version")
	}

	var release GitHubRelease
	err = json.NewDecoder(resp.Body).Decode(&release)
	if err != nil {
		fmt.Println("Error decoding response:", err)
		return "", err

	}

	return release.TagName, nil
}

// returns the temp path at which we unpacked the archive
func downloadArchive(url string) (string, error) {
	tempDir, err := os.MkdirTemp("", "connectinternat")
	if err != nil {
		return "", fmt.Errorf("error creating tempdir, %w", err)
	}
	switch getArchiveExtension(url) {
	case ".zip":
		return tempDir, DownloadAndExtractZip(url, tempDir)
	case ".gz":
		return tempDir, DownloadAndExtractTarGz(url, tempDir)
	}
	return "", fmt.Errorf("invalid file type")
}

func getArchiveExtension(url string) string {
	return filepath.Ext(url)
}
