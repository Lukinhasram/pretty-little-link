import { useState, useCallback } from "react";
import { createShortURL } from "../services/api";

export const useShortener = () => {
    const [shortURL, setShortURL] = useState("");
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState(null);
    
    const generateShortURL = useCallback(async (originalURL) => {
        setIsLoading(true);
        setShortURL("");
        setError(null);

        try {
            const newShortURL = await createShortURL(originalURL);
            setShortURL(newShortURL);
        } catch (err) {
            setError(err.message);
        } finally {
            setIsLoading(false);
        }
    }, []);

    return { shortURL, isLoading, error, generateShortURL }
};