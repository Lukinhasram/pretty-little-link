const API_URL = "http://localhost:3000";

// Calls the backend API to create a short URL
// It sends the original URL as a JSON object and expects a short URL in response.
// If the request fails, it throws an error with the message from the response or a default message.

export const createShortURL = async (originalURL) => {
    const response = await fetch(`${API_URL}/shorten`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ original_url: originalURL }),
    });

    const data = await response.json();
    
    if (!response.ok) {
        throw new Error(data.error || 'Failed to create short URL');
    }

    return data.short_url;
}