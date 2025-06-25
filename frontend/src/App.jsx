import { useState } from 'react';
import './App.css';

function App() {
  // State hooks to manage the form input, API results, and loading/error states.
  const [originalUrl, setOriginalUrl] = useState('');
  const [shortUrl, setShortUrl] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  // This function is called when the user submits the form.
  const handleSubmit = async (event) => {
    event.preventDefault(); // Prevents the browser from reloading the page.
    
    // Reset state for a new submission.
    setShortUrl('');
    setError(null);
    setIsLoading(true);

    try {
      // Use the fetch API to send a POST request to our Rust backend.
      // This URL works because our backend is running in Docker and mapped to port 3000.
      const response = await fetch('http://localhost:3000/shorten', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ original_url: originalUrl }),
      });

      const data = await response.json();

      if (!response.ok) {
        // Handle errors returned from the API (e.g., invalid URL).
        throw new Error(data.error || 'Failed to create short link.');
      }

      setShortUrl(data.short_url); // On success, store the returned short URL.

    } catch (err) {
      setError(err.message); // Store any network or API error message.
    } finally {
      setIsLoading(false); // Always stop the loading indicator.
    }
  };

  return (
    <div className="container">
      <h1>Rust Link Shortener</h1>
      <p className="subtitle">A fast and simple URL shortener powered by Rust + React</p>
      <form onSubmit={handleSubmit} className="url-form">
        <input
          type="url"
          placeholder="Enter a long URL to shorten..."
          value={originalUrl}
          onChange={(e) => setOriginalUrl(e.target.value)}
          required
          className="url-input"
        />
        <button type="submit" disabled={isLoading} className="submit-button">
          {isLoading ? 'Shortening...' : 'Shorten'}
        </button>
      </form>

      {/* Conditionally render the result, loading, or error messages */}
      {shortUrl && (
        <div className="result-box">
          <p>Your shortened URL:</p>
          <a href={shortUrl} target="_blank" rel="noopener noreferrer">
            {shortUrl}
          </a>
        </div>
      )}

      {error && (
        <div className="error-box">
          <p>{error}</p>
        </div>
      )}
    </div>
  );
}

export default App;
