import React, {useState} from "react";

function URLForm ({ onShorten, isLoading }) {
    console.log('  -> Rendering URLForm component...');

    const [originalURL, setOriginalURL] = useState("");

    const handleSubmit = (event) => {
        event.preventDefault();

        onShorten(originalURL);
    };

    return (
        <form onSubmit={handleSubmit} className="url-form">
            <input 
                type="url" 
                placeholder="Enter a long URL to shorten..."
                value={originalURL}
                onChange={(e) => setOriginalURL(e.target.value)}
                required
                className="url-input"
            />
            <button type="submit" disabled={isLoading} className="submit-button">
                {isLoading ? "Shortening..." : "Shorten"}
            </button>
        </form>
    )
}

export default URLForm