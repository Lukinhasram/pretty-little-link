import React from "react";

function ResultDisplay({shortURL, error}) {
    if (shortURL) {
        return (
            <div className="result-box">
                <p>Your shortened URL: </p>
                    <a href={shortURL} target="_blank" rel="noopener noreferrer">
                    {shortURL}
                </a>
            </div>
        );
    }

    if (error) {
        return (
            <div className="error-box">
                <p>{error}</p>
            </div>
        );
    }

    return null;
}

export default ResultDisplay;