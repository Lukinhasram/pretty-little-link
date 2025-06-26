import "./App.css";

import Header from "./components/Header";
import URLForm from "./components/URLForm";
import ResultDisplay from "./components/ResultDisplay";

import mouseImage from "./assets/mouse_image.png";

import { useShortener } from "./hooks/useShortener";
import { useEffect, useState } from "react";

function App() {
    const { shortURL, isLoading, error, generateShortURL } = useShortener();
    const [theme, setTheme] = useState(() => localStorage.getItem("theme") || "light");

    useEffect(() =>{
        document.body.className = theme === "light" ? "light-mode" : "";
        localStorage.setItem("theme", theme);
    }, [theme]);

    const toggleTheme = () => {
        setTheme((prevTheme) => (prevTheme === "dark" ? "light" : "dark"));
    };

    return (
        <>
            <button onClick={toggleTheme} className="theme-toggle-button">
                {theme === 'dark' ? (
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>
                ) : (
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>

                )}
            </button>

            <div className="container">
                <Header/>
                <URLForm onShorten={generateShortURL} isLoading={isLoading} />
                <ResultDisplay shortURL={shortURL} error={error} />
            </div>

            <a href="https://github.com/Lukinhasram" target="_blank" rel="noopener noreferrer" className="github-link-container">
                <span className="github-popup-text">Check out my github!</span>                
                <img
                    src={mouseImage}
                    alt="Mouse animal using a computer mouse"
                    className="mouse-image"
                />
            </a>
        </>
    );
}

export default App;