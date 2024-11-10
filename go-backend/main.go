package main

import (
	"bufio"
	"bytes"
	"crypto/tls"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
	"os"
	"strconv"
	"time"
)

const loginURL = "https://controller.access.network/portal_api.php"

func main() {
	// entry: [n]
	// [arg1] (aka the reqType)
	// [arg2]
	// ...
	// [argn]
	scanner := bufio.NewScanner(os.Stdin)
	var n int
	var err error
	scanner.Scan()
	n, err = strconv.Atoi(scanner.Text())
	if err != nil {
		fmt.Println("Couldn't find n")
		os.Exit(1)
		return
	}

	if n < 1 {
		fmt.Println("n too small")
		os.Exit(1)
		return
	}

	args := make([]string, n)
	for i := range args {
		scanner.Scan()
		args[i] = scanner.Text()
	}

	reqType := args[0]
	switch reqType {
	case "login":
		if n != 3 {
			fmt.Println("n too small: want 2 arguments, the username and the password")
			os.Exit(1)
			return
		}
		client := Client{}
		client.Login(args[1], args[2])
		break
	case "logout", "ping":
		if n != 3 {
			fmt.Println("n too small: want 2 arguments, the username and digest")
			os.Exit(1)
			return
		}
		client := Client{}
		if reqType == "logout" {
			client.Logout(args[1], args[2])
		} else if reqType == "ping" {
			client.Ping(args[1], args[2])
		}
		break
	default:
		fmt.Println("invalid request type")
		os.Exit(1)
		return
	}
}

// =================================================================================================
// =======================          CLIENT CODE                                    	================
// =================================================================================================

type LoginResponse struct {
	// AuthenticateStep any    `json:"authenticate_step"`
	// AuthenticateType any    `json:"authenticate_type"`
	Step string `json:"step"`
	Type string `json:"type"`
	User struct {
		Login struct {
			Value string `json:"value"`
		} `json:"login"`
		PasswordDigest struct {
			Value string `json:"value"`
		} `json:"passwordDigest"`
		IPAddress struct {
			Value string `json:"value"`
		} `json:"ipAddress"`
		Profile struct {
			Value string `json:"value"`
		} `json:"profile"`
		Services struct {
			Value string `json:"value"`
		} `json:"services"`
		AutoDisconnect struct {
			Value bool `json:"value"`
		} `json:"autoDisconnect"`
		Schedule struct {
			Value []struct {
				Begin struct {
					Day  string `json:"day"`
					Hour string `json:"hour"`
					Min  string `json:"min"`
				} `json:"begin"`
				End struct {
					Day  string `json:"day"`
					Hour string `json:"hour"`
					Min  string `json:"min"`
				} `json:"end"`
			} `json:"value"`
		} `json:"schedule"`
		Validity struct {
			Value string `json:"value"`
		} `json:"validity"`
		InitTimeGMT struct {
			Value string `json:"value"`
		} `json:"initTimeGMT"`
		TimeCredit struct {
			Value     string `json:"value"`
			Remaining struct {
				Value int `json:"value"`
			} `json:"remaining"`
			Reneweach struct {
				Value string `json:"value"`
			} `json:"reneweach"`
			InitialRemaining struct {
				Value int `json:"value"`
			} `json:"initialRemaining"`
		} `json:"timeCredit"`
		IncomingNetwork struct {
			Value string `json:"value"`
		} `json:"incomingNetwork"`
		IncomingNetworkID struct {
			Value string `json:"value"`
		} `json:"incomingNetworkID"`
		IncomingZone struct {
			Value string `json:"value"`
		} `json:"incomingZone"`
		IncomingVlan struct {
			Value string `json:"value"`
		} `json:"incomingVlan"`
		IncommingVlan struct {
			Value string `json:"value"`
		} `json:"incommingVlan"`
		IncommingZone struct {
			Value string `json:"value"`
		} `json:"incommingZone"`
		Multidevice struct {
			Value string `json:"value"`
		} `json:"multidevice"`
		UniversalTime struct {
			Value int `json:"value"`
		} `json:"universalTime"`
		TimezoneOffset struct {
			Value string `json:"value"`
		} `json:"timezoneOffset"`
		RequestedURL struct {
			Value string `json:"value"`
		} `json:"requestedURL"`
		AllowModPwdBySelf  bool `json:"allowModPwdBySelf"`
		GetPurchaseSummary struct {
			Show bool `json:"show"`
		} `json:"getPurchaseSummary"`
	} `json:"user"`
}

type Client struct {
}

func (c *Client) newUnsecureHTTPClient() *http.Client {
	tlsConfig := &tls.Config{
		MinVersion:         tls.VersionTLS10,
		MaxVersion:         tls.VersionTLS10,
		InsecureSkipVerify: true,
	}
	// Create a new HTTP client with the TLS configuration
	client := &http.Client{
		Transport: &http.Transport{
			TLSClientConfig: tlsConfig,
		},
	}
	return client
}

func (c *Client) Login(username string, password string) error {
	client := c.newUnsecureHTTPClient()

	body := []byte(fmt.Sprintf(`action=authenticate&login=%s&password=%s&policy_accept=false`, url.QueryEscape(username), url.QueryEscape(password)))

	r, err := http.NewRequest("POST", loginURL, bytes.NewBuffer(body))
	r.Header.Add("Content-Type", "application/x-www-form-urlencoded")

	resp, err := client.Do(r)
	if err != nil {
		fmt.Println("Error making request:", err)
		return err
	}
	defer resp.Body.Close()

	bodyBytes, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error reading body:", err)
		return err
	}

	if resp.StatusCode != 200 {
		fmt.Println("[login] Error response status code:", resp.StatusCode)
		fmt.Println("[login] Response body:", string(bodyBytes))
		return fmt.Errorf("[login] Error response status code: %d", resp.StatusCode)
	}

	var loginResponse LoginResponse
	err = json.Unmarshal(bodyBytes, &loginResponse)
	if err != nil {
		fmt.Println("Error unmarshalling response:", err)
		return err
	}
	// check that the stuff is actually defined till the password digest
	username = loginResponse.User.Login.Value
	passwordDigest := loginResponse.User.PasswordDigest.Value

	fmt.Println(username)
	fmt.Println(passwordDigest)
	return nil
}

func (c *Client) Ping(username string, passwordDigest string) error {
	client := c.newUnsecureHTTPClient()

	body := []byte(fmt.Sprintf(`action=refresh&login=%s&password_digest=%s&policy_accept=false`, url.QueryEscape(username), url.QueryEscape(passwordDigest)))

	r, err := http.NewRequest("POST", loginURL, bytes.NewBuffer(body))
	r.Header.Add("Content-Type", "application/x-www-form-urlencoded")

	resp, err := client.Do(r)
	if err != nil {
		fmt.Println("Error making request:", err)
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		bodyBytes, err := ioutil.ReadAll(resp.Body)
		if err != nil {
			fmt.Println("Error reading body:", err)
			return err
		}

		fmt.Println("[pinging] Error response status code:", resp.StatusCode)
		fmt.Println("[pinging] Response body:", string(bodyBytes))
		return fmt.Errorf("[pinging] Error response status code: %d", resp.StatusCode)
	}

	return nil
}

func (c *Client) Logout(username string, passwordDigest string) error {
	client := c.newUnsecureHTTPClient()

	body := []byte(fmt.Sprintf(`action=disconnect&login=%s&password_digest=%s`, url.QueryEscape(username), url.QueryEscape(passwordDigest)))

	r, err := http.NewRequest("POST", loginURL, bytes.NewBuffer(body))
	r.Header.Add("Content-Type", "application/x-www-form-urlencoded")

	resp, err := client.Do(r)
	if err != nil {
		fmt.Println("Error making request:", err)
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		bodyBytes, err := ioutil.ReadAll(resp.Body)
		if err != nil {
			fmt.Println("Error reading body:", err)
			return err
		}

		fmt.Println("[logout] Error response status code:", resp.StatusCode)
		fmt.Println("[logout] Response body:", string(bodyBytes))
		return fmt.Errorf("[logout] Error response status code: %d", resp.StatusCode)
	}

	return nil
}

func (c *Client) StartTicking(username string, passwordDigest string) error {
	interval := 50
	go func() {
		ticker := time.NewTicker(time.Second * time.Duration(interval))
		for range ticker.C {
			c.Ping(username, passwordDigest)
		}
	}()
	return nil
}
