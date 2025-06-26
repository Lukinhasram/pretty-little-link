import React from "react";

function Header() {
    console.log('  -> Rendering Header component...');
    return (
        <header>
            <h1>Pretty Little Link</h1>
            <p className="subtittle">A fast and simple URL shortener made with <b>Rust</b></p>
        </header>
    );
}

export default Header;