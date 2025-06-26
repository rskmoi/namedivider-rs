package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

// DividedName represents a divided name result
type DividedName struct {
	Family    string  `json:"family"`
	Given     string  `json:"given"`
	Separator string  `json:"separator"`
	Score     float64 `json:"score"`
	Algorithm string  `json:"algorithm"`
}

// DivideRequest represents the request payload for name division
type DivideRequest struct {
	Names []string `json:"names"`
	Mode  string   `json:"mode,omitempty"`
}

// DivideResponse represents the response from the API
type DivideResponse struct {
	DividedNames []DividedName `json:"divided_names"`
}

// NameDividerClient is the SDK client for NameDivider API
type NameDividerClient struct {
	BaseURL    string
	HTTPClient *http.Client
}

// NewNameDividerClient creates a new NameDivider client
func NewNameDividerClient(baseURL string) *NameDividerClient {
	if baseURL == "" {
		baseURL = "http://localhost:8000"
	}
	return &NameDividerClient{
		BaseURL:    baseURL,
		HTTPClient: &http.Client{},
	}
}

// Divide divides names using the specified mode
func (c *NameDividerClient) Divide(names []string, mode string) ([]DividedName, error) {
	if mode == "" {
		mode = "basic"
	}

	request := DivideRequest{
		Names: names,
		Mode:  mode,
	}

	jsonData, err := json.Marshal(request)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %w", err)
	}

	resp, err := c.HTTPClient.Post(
		c.BaseURL+"/divide",
		"application/json",
		bytes.NewBuffer(jsonData),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to make request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("API returned status code: %d", resp.StatusCode)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %w", err)
	}

	var response DivideResponse
	if err := json.Unmarshal(body, &response); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return response.DividedNames, nil
}

// DivideBasic divides names using BasicNameDivider
func (c *NameDividerClient) DivideBasic(names []string) ([]DividedName, error) {
	return c.Divide(names, "basic")
}

// DivideGBDT divides names using GBDTNameDivider
func (c *NameDividerClient) DivideGBDT(names []string) ([]DividedName, error) {
	return c.Divide(names, "gbdt")
}